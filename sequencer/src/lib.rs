pub mod metronome;
pub mod oscillator;
pub mod synthesizer;
pub mod sampler;
pub mod adsr;
pub mod decibels;
pub mod midimessage;
pub mod processor;
pub mod fifoqueue;
pub mod noise;

use crate::processor::Processor;
use crate::metronome::metronome::Metronome;
use crate::synthesizer::synthesizer::Synthesizer;
use crate::midimessage::MidiMessage;
use crate::fifoqueue::FifoQueue;
use crate::midimessage::NOTE_ON;
use crate::midimessage::NOTE_OFF;

pub enum Message {
    PresetPrev,
    PresetNext,
    Midi(MidiMessage),
}

pub struct Sequencer {
    pub sample_rate: f32,
    pub tick: i32,
    pub bars: i32,
    tick_time: f32,
    ticks_per_quarter_note: i32,
    time_accumulated: f32,
    tempo: f32,
    volume: f32,
    elapsed_time_each_render: f32,
    metronome: Metronome,
    pub metronome_active: bool,
    pub buffer_size: usize,
    pub processors: Vec<Box<dyn Processor + Send>>,
    pub instrument_selected_id: usize,
    pub is_recording: bool,
    pub fifo_queue_midi_message: FifoQueue<MidiMessage>,
    pub bpm_has_biped: bool,
}

impl Sequencer {
    pub fn new(sample_rate: f32, buffer_size: usize) -> Sequencer {
        
        let metronome = Metronome::new(sample_rate);

        let mut sequencer = Sequencer {
            sample_rate: sample_rate,
            tick: 0,
            tick_time: 0.0,
            ticks_per_quarter_note: 960,
            time_accumulated: 0.0,
            tempo: 10.0,
            volume: 0.6,
            elapsed_time_each_render: 0.0,
            metronome: metronome,
            metronome_active: false,
            buffer_size: buffer_size,
            processors: Vec::new(),
            instrument_selected_id: 0,
            bars: 2,
            is_recording: false,
            fifo_queue_midi_message: FifoQueue::new(64),
            bpm_has_biped: false
        };
        sequencer.set_tempo(90.0);
        sequencer.compute_elapsed_time_each_render();

        sequencer.processors.push(Box::new(Synthesizer::new(sample_rate, 0, 0)));
        sequencer.processors.push(Box::new(Synthesizer::new(sample_rate, 1, 1)));
        sequencer.processors.push(Box::new(Synthesizer::new(sample_rate, 2, 2)));

        return sequencer;
    }

    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = tempo;
        self.compute_tick_time();
    }

    pub fn set_bars(&mut self, bars: i32) {
        self.tick = 0;
        self.bars = bars;
    }

    pub fn set_is_recording(&mut self, is_recording: bool) {
        self.is_recording = is_recording;
    }
    
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
    
    pub fn compute_tick_time(&mut self) {
        self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
    }

    pub fn compute_elapsed_time_each_render(&mut self) {
        self.elapsed_time_each_render = self.buffer_size as f32 / self.sample_rate as f32
    }

    pub fn metronomome_tick(&mut self) -> bool {
        if self.tick % self.ticks_per_quarter_note == 0 {
            let start_bar = self.tick % (self.ticks_per_quarter_note * 4) == 0;
            if self.metronome_active {
                self.metronome.bip(start_bar);
            }
            return true;
        }
        return false;
    }

    pub fn handle_incoming_note_events(&mut self) {
        loop {
            let midi_message = self.fifo_queue_midi_message.read();
            if let Some(midi_message) = midi_message {
                for i in 0..self.processors.len() {
                    if self.processors[i].is_armed() {
                        self.processors[i].add_notes_event(*midi_message); 
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn play_recorded_note_events(&mut self) {
        for i in 0..self.processors.len() {
            for k in 0..self.processors[i].get_notes_events().len() {
                if self.processors[i].get_notes_events()[k].tick == self.tick {
                    if self.processors[i].get_notes_events()[k].first == NOTE_OFF as u8 {
                        let note_id = self.processors[i].get_notes_events()[k].second;
                        self.processors[i].note_off(note_id);
                    }
                    if self.processors[i].get_notes_events()[k].first == NOTE_ON as u8 {
                        let note_id = self.processors[i].get_notes_events()[k].second;
                        self.processors[i].note_on(note_id, 1.0);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.bpm_has_biped = false;
        self.time_accumulated += self.elapsed_time_each_render;
        while self.time_accumulated >= self.tick_time {
            self.time_accumulated -= self.tick_time;
            
          
            if !self.bpm_has_biped {
                self.bpm_has_biped = self.metronomome_tick();
            }

            self.play_recorded_note_events();

            self.tick += 1;

            if self.tick >= self.bars * self.ticks_per_quarter_note * 4 {
                self.tick = 0;
            }
        }
        self.handle_incoming_note_events();
    }

    pub fn process(&mut self, outputs: *mut f32, num_samples: usize, nb_channels: usize) {
        self.update();

        for s in 0..(nb_channels * num_samples) {
            unsafe { *outputs.offset(s as isize) = 0.0; }
        }

        self.metronome.process(outputs, num_samples, nb_channels);
        
        for i in 0..self.processors.len() {
            self.processors[i].process(outputs, num_samples, nb_channels);
        }

        for s in 0..(nb_channels * num_samples) {
            unsafe { *outputs.offset(s as isize) *= self.volume; }
        }
    }

    pub fn get_tick(&self) -> i32 {
        return self.tick;
    }

    pub fn note_on(&mut self, note_id: u8) {
        for i in 0..self.processors.len() {
            if self.processors[i].is_armed() {
                self.processors[i].note_on(note_id, 1.0);
                if self.is_recording {
                    self.fifo_queue_midi_message.write(
                        MidiMessage {
                            first: NOTE_ON,
                            second: note_id,
                            third: 127,
                            tick: self.tick
                        }
                    );
                }
            }
        }
    }

    pub fn note_off(&mut self, note_id: u8) {
        for i in 0..self.processors.len() {
            if self.processors[i].is_armed() {
                self.processors[i].note_off(note_id);
                if self.is_recording {
                    self.fifo_queue_midi_message.write(
                        MidiMessage {
                            first: NOTE_OFF,
                            second: note_id,
                            third: 127,
                            tick: self.tick
                        }
                    );
                }
            }
        }
    }

    pub fn clear_notes_events(&mut self, clear_all_instruments: bool) {
        for i in 0..self.processors.len() {
            if self.processors[i].is_armed() || clear_all_instruments {
                self.processors[i].clear_notes_events();
            }
        }
    }

    pub fn next_instrument(&mut self) {
        self.instrument_selected_id += 1;
        if self.instrument_selected_id > self.processors.len() - 1 {
            self.instrument_selected_id = 0;
        }
        for (i, proc) in self.processors.iter_mut().enumerate() {
            proc.set_is_armed(i == self.instrument_selected_id);
        }
    } 
    
    pub fn previous_instrument(&mut self) {
        if self.instrument_selected_id > 0 {
            self.instrument_selected_id -= 1;
        } else {
            self.instrument_selected_id = self.processors.len() - 1;
        }
        for (i, proc) in self.processors.iter_mut().enumerate() {
            proc.set_is_armed(i == self.instrument_selected_id);
        }
    } 
}
