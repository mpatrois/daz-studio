use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

#[derive(Copy, Clone)]
pub enum Message {
    SetTempo(f32),
    SetVolume(f32),
    SetMetronomeActive(bool),
    NextInstrument,
    PreviousInstrument,
    NextPreset,
    PreviousPreset,
    SetIsRecording(bool),
    SetCurrentInstrumentSelected(usize),
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
}

pub struct SequencerData {
    pub tempo: f32,
    pub tick: i32,
    pub tick_time: f32,
    pub volume: f32,
    pub metronome_active: bool,
    pub ticks_per_quarter_note: i32,
    pub is_recording: bool,
    pub instrument_selected_id: usize,
    pub insruments: Vec<InstrumentData>,
    pub receiver: Receiver<Message>,
}

impl SequencerData {
    
    pub fn new() -> (SequencerData, Sender<Message>) {
        let (sender, receiver) = mpsc::channel::<Message>();
        let mut data = SequencerData {
            tick: 0,
            tempo: 90.0,
            volume: 0.6,
            metronome_active: false,
            is_recording: false,
            ticks_per_quarter_note: 960,
            instrument_selected_id: 0,
            tick_time: 0.0,
            insruments: Vec::new(),
            receiver
        };
        data.compute_tick_time();
        (data, sender)
    }

    pub fn process_messages(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                Message::SetTempo(x) => {
                    self.tempo = x;
                    self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
                },
                Message::SetMetronomeActive(x) => {
                    self.metronome_active = x;
                },
                Message::SetIsRecording(x) => {
                    self.is_recording = x;
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
                _ => (),
            }
        }
    }

    fn compute_tick_time(&mut self) {
        self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
    }
}