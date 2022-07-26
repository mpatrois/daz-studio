use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;

use serde::{Deserialize, Serialize};

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
    SetWaveFormData(Vec<f32>),
    SetRMSInstrument(usize, f32, f32),
    NextInstrument,
    PreviousInstrument,
    PlayStop,
    NextPreset,
    PreviousPreset,
    NextQuantize,
    PreviousQuantize,
    SetIsRecording(bool),
    SetCurrentInstrumentSelected(usize),
    UndoLastSession,
    SetInstruments(Vec<InstrumentData>),
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

#[derive(Clone, Deserialize, Serialize)]
pub struct InstrumentData {
    pub name: String,
    pub volume: f32,
    pub current_preset_id: usize,
    pub presets: Vec<String>,
    pub paired_notes: Vec<NoteEvent>,
    pub rms_left: f32,
    pub rms_right: f32,
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
    pub instruments: Vec<InstrumentData>,
    pub receiver: Receiver<Message>,
    pub record_session: i32,
    pub undo_last_session: bool,
    pub kill_all_notes: bool,
    pub audio_wave_form: Vec<f32>,
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
            volume: 1.,
            metronome_active: true,
            is_recording: true,
            ticks_per_quarter_note: 960,
            instrument_selected_id: 0,
            tick_time: 0.0,
            instruments: Vec::new(),
            receiver,
            record_session: 0,
            undo_last_session: false,
            kill_all_notes: false,
            audio_wave_form: Vec::new()
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
                    if self.instrument_selected_id > self.instruments.len() - 1 {
                        self.instrument_selected_id = 0;
                    }
                },
                Message::PreviousInstrument => {
                    if self.instrument_selected_id > 0 {
                        self.instrument_selected_id -= 1;
                    } else {
                        self.instrument_selected_id = self.instruments.len() - 1;
                    }
                },
                Message::NextPreset => {
                    self.instruments[self.instrument_selected_id].current_preset_id += 1;
                    if self.instruments[self.instrument_selected_id].current_preset_id > self.instruments[self.instrument_selected_id].presets.len() - 1 {
                        self.instruments[self.instrument_selected_id].current_preset_id = 0;
                    }
                },
                Message::PreviousPreset => {
                    if  self.instruments[self.instrument_selected_id].current_preset_id > 0  {
                        self.instruments[self.instrument_selected_id].current_preset_id -= 1;
                    } else {
                        self.instruments[self.instrument_selected_id].current_preset_id = self.instruments[self.instrument_selected_id].presets.len() - 1;
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
                    self.instruments[self.instrument_selected_id].paired_notes = note_events;
                },
                Message::UndoLastSession => {
                    self.undo_last_session = true;
                },
                Message::SetWaveFormData(audio_wave_form) => {
                    self.audio_wave_form = audio_wave_form;
                },
                Message::SetRMSInstrument(idx, rms_left, rms_right) => {
                    self.instruments[idx].rms_left = rms_left;
                    self.instruments[idx].rms_right = rms_right;
                },
                Message::SetInstruments(instruments) => {
                    self.instruments = instruments;
                    for instrument in self.instruments.iter_mut() {
                        for note_event in instrument.paired_notes.iter_mut() {
                            note_event.record_session = -1;
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

    pub fn import_from_file(&mut self, filepath: String) -> Result<Vec<InstrumentData>, Box<dyn Error>>  {
        
        let file = File::open(&std::path::Path::new(&filepath))?;
        let reader = BufReader::new(file);

        let instruments: Vec<InstrumentData> = serde_json::from_reader(reader)?;

        Ok(instruments)
    }
    
    pub fn export_to_file(&mut self, filepath: String) -> Result<(), Box<dyn Error>>  {
        
        // println!("Export");
        let parent_path = std::path::Path::new(&filepath).parent().unwrap().to_str().unwrap();
        fs::create_dir_all(parent_path)?;
        let mut file = File::create(&std::path::Path::new(&filepath))?;
        let serialized = serde_json::to_string(&self.instruments).unwrap();
        file.write_all(serialized.as_bytes())?;
        println!("Export");
        Ok(())
    }
}