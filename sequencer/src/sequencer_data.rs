use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use crate::midimessage::NoteEvent;

const QUANTIZE_VALUE: [i32; 8] = [-1, 2, 4, 8, 16, 32, 64, 128];

#[derive(Clone)]
pub enum Message {
    SetTempo(f32),
    SetTick(i32),
    SetVolume(f32),
    SetMetronomeActive(bool),
    SetBpmHasBiped(bool),
    SetMidiMessagesInstrument(Vec<NoteEvent>),
    NextInstrument,
    PreviousInstrument,
    PlayStop,
    NextPreset,
    PreviousPreset,
    NextQuantize,
    PreviousQuantize,
    SetIsRecording(bool),
    DumpNotes,
    SetCurrentInstrumentSelected(usize),
    UndoLastSession,
}

#[derive(Clone)]
pub struct DataBroadcaster {
    pub senders: Vec<mpsc::Sender<Message>>
}

impl DataBroadcaster {
    pub fn send(&self, msg: Message) {
        for sender in self.senders.iter() {
            sender.send(msg.clone()).unwrap();
        }
    }
}

#[derive(Clone)]
pub struct InstrumentData {
    pub name: String,
    pub volume: f32,
    pub current_preset_id: usize,
    pub presets: Vec<String>,
    pub paired_notes: Vec<NoteEvent>
}

pub struct SequencerData {
    pub tempo: f32,
    pub quantize_idx: usize,
    pub tick: i32,
    pub bpm_has_biped: bool,
    pub bars: i32,
    pub is_playing: bool,
    pub tick_time: f32,
    pub volume: f32,
    pub metronome_active: bool,
    pub ticks_per_quarter_note: i32,
    pub is_recording: bool,
    pub instrument_selected_id: usize,
    pub insruments: Vec<InstrumentData>,
    pub receiver: Receiver<Message>,
    pub record_session: i32,
    pub undo_last_session: bool,
    pub kill_all_notes: bool,
}

impl SequencerData {
    
    pub fn new() -> (SequencerData, Sender<Message>) {
        let (sender, receiver) = mpsc::channel::<Message>();
        let mut data = SequencerData {
            tick: 0,
            tempo: 95.0,
            quantize_idx: 2,
            bars: 2,
            is_playing: false,
            bpm_has_biped: false,
            volume: 0.6,
            metronome_active: true,
            is_recording: true,
            ticks_per_quarter_note: 960,
            instrument_selected_id: 0,
            tick_time: 0.0,
            insruments: Vec::new(),
            receiver,
            record_session: 0,
            undo_last_session: false,
            kill_all_notes: false,
        };
        data.compute_tick_time();
        (data, sender)
    }

    pub fn process_messages(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                Message::PlayStop => {
                    self.is_playing = !self.is_playing;
                    self.tick = 0;
                    self.record_session += 1;
                    if !self.is_playing {
                        self.kill_all_notes = true;
                    }
                },
                Message::SetTempo(x) => {
                    self.tempo = x;
                    self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
                },
                Message::SetMetronomeActive(x) => {
                    self.metronome_active = x;
                },
                Message::SetTick(x) => {
                    self.tick = x;
                },
                Message::SetIsRecording(x) => {
                    self.is_recording = x;
                    self.record_session += 1;
                },
                Message::SetBpmHasBiped(x) => {
                    self.bpm_has_biped = x;
                },
                Message::NextInstrument => {
                    self.instrument_selected_id += 1;
                    if self.instrument_selected_id > self.insruments.len() - 1 {
                        self.instrument_selected_id = 0;
                    }
                },
                Message::PreviousInstrument => {
                    if self.instrument_selected_id > 0 {
                        self.instrument_selected_id -= 1;
                    } else {
                        self.instrument_selected_id = self.insruments.len() - 1;
                    }
                },
                Message::NextPreset => {
                    self.insruments[self.instrument_selected_id].current_preset_id += 1;
                    if self.insruments[self.instrument_selected_id].current_preset_id > self.insruments[self.instrument_selected_id].presets.len() - 1 {
                        self.insruments[self.instrument_selected_id].current_preset_id = 0;
                    }
                },
                Message::PreviousPreset => {
                    if  self.insruments[self.instrument_selected_id].current_preset_id > 0  {
                        self.insruments[self.instrument_selected_id].current_preset_id -= 1;
                    } else {
                        self.insruments[self.instrument_selected_id].current_preset_id = self.insruments[self.instrument_selected_id].presets.len() - 1;
                    }
                },
                Message::NextQuantize => {
                    self.quantize_idx += 1;
                    if self.quantize_idx > QUANTIZE_VALUE.len() - 1 {
                        self.quantize_idx = 0;
                    }
                },
                Message::PreviousQuantize => {
                    if self.quantize_idx > 0  {
                        self.quantize_idx -= 1;
                    } else {
                        self.quantize_idx = QUANTIZE_VALUE.len() - 1;
                    }
                },
                Message::SetMidiMessagesInstrument(note_events) => {
                    self.insruments[self.instrument_selected_id].paired_notes = note_events;
                },
                Message::UndoLastSession => {
                    self.undo_last_session = true;
                },
                Message::DumpNotes => {
                    for intrument in self.insruments.iter()  {
                        println!("Instrument: {}", intrument.name);
                        for noteEvent in intrument.paired_notes.iter() {
                            println!("{{");
                            println!("note_id: {},", noteEvent.note_id);
                            println!("tick_on: {},", noteEvent.tick_on);
                            println!("tick_off: {},", noteEvent.tick_off);
                            println!("}},");
                        }
                    }
                },
                _ => (),
            }
        }
    }

    fn compute_tick_time(&mut self) {
        self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
    }
    
    pub fn nb_ticks(&self) -> i32 {
        self.bars * 4 * self.ticks_per_quarter_note
    }
    
    pub fn end_quantize_adjust(&self) -> i32 {
        self.nb_ticks() - 120
    }

    pub fn quantize_interval(&self) -> i32 {
       ((1. / self.get_quantize() as f32) * self.ticks_per_quarter_note as f32) as i32
    }
    
    pub fn get_quantize(&self) -> i32 {
       QUANTIZE_VALUE[self.quantize_idx]
    }
}