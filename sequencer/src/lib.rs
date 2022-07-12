pub mod metronome;
pub mod oscillator;
pub mod synthesizer;
pub mod adsr;
pub mod decibels;
pub mod midimessage;
pub mod processor;
pub mod fifoqueue;
pub mod sampler;
pub mod noise;
pub mod preset;
pub mod sequencer_data;

use crate::processor::Processor;
use crate::metronome::metronome::Metronome;
use crate::synthesizer::synthesizer::Synthesizer;
use crate::sampler::sampler::Sampler;
use crate::midimessage::MidiMessage;
use crate::fifoqueue::FifoQueue;
use crate::sequencer_data::SequencerData;
use crate::sequencer_data::InstrumentData;
use crate::sequencer_data::Message as SequencerDataMessage;
use crate::midimessage::NOTE_ON;
use crate::midimessage::NOTE_OFF;
use std::sync::mpsc::Sender;

pub enum Message {
    Midi(MidiMessage),
}

pub struct Sequencer {
    pub data: SequencerData,
    sample_rate: f32,
    time_accumulated: f32,
    elapsed_time_each_render: f32,
    metronome: Metronome,
    buffer_size: usize,
    processors: Vec<Box<dyn Processor>>,
    fifo_queue_midi_message: FifoQueue<MidiMessage>,
    bpm_has_biped: bool,
    pub audio_state_senders: Vec<Sender<sequencer_data::Message>>
}

impl Sequencer {
    pub fn new(sample_rate: f32, buffer_size: usize) -> (Sequencer, Sender<sequencer_data::Message>) {
        
        let metronome = Metronome::new(sample_rate);

        let (data, sender) = SequencerData::new();

        let mut sequencer = Sequencer {
            sample_rate: sample_rate,
            time_accumulated: 0.0,
            elapsed_time_each_render: 0.0,
            metronome: metronome,
            buffer_size: buffer_size,
            processors: Vec::new(),
            fifo_queue_midi_message: FifoQueue::new(64),
            bpm_has_biped: false,
            data,
            audio_state_senders: Vec::new(),
        };

        sequencer.compute_elapsed_time_each_render();

        sequencer.processors.push(Box::new(Sampler::new(sample_rate, 0)));
        sequencer.processors.push(Box::new(Synthesizer::new(sample_rate, 1, 0)));
        sequencer.processors.push(Box::new(Synthesizer::new(sample_rate, 2, 1)));
        sequencer.processors.push(Box::new(Synthesizer::new(sample_rate, 3, 2)));

        sequencer.processors[0].set_is_armed(true);

        for processor in sequencer.processors.iter() {
            sequencer.data.insruments.push(InstrumentData {
                name: processor.get_name(),
                volume: 1.0,
                current_preset_id: processor.get_current_preset_id(),
                presets: processor.get_presets().iter().map(|preset| preset.get_name()).collect(),
            });
        }
        return (sequencer, sender);
    }

    pub fn compute_elapsed_time_each_render(&mut self) {
        self.elapsed_time_each_render = self.buffer_size as f32 / self.sample_rate as f32
    }

    pub fn metronomome_tick(&mut self) -> bool {
        if self.data.tick % self.data.ticks_per_quarter_note == 0 {
            let start_bar = self.data.tick % (self.data.ticks_per_quarter_note * 4) == 0;
            if self.data.metronome_active {
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
                let idx = self.data.instrument_selected_id;
                if self.data.instrument_selected_id < self.processors.len() {
                    self.processors[idx].add_notes_event(*midi_message); 
                }
            } else {
                break;
            }
        }
    }

    pub fn play_recorded_note_events(&mut self) {
        for i in 0..self.processors.len() {
            for k in 0..self.processors[i].get_notes_events().len() {
                if self.processors[i].get_notes_events()[k].tick == self.data.tick {
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
        while self.time_accumulated >= self.data.tick_time {
            self.time_accumulated -= self.data.tick_time;
          
            if !self.bpm_has_biped {
                self.bpm_has_biped = self.metronomome_tick();
            }

            self.play_recorded_note_events();

            self.data.tick += 1;

            if self.data.tick >= self.data.bars * self.data.ticks_per_quarter_note * 4 {
                self.data.tick = 0;
            }
        }
        self.handle_incoming_note_events();
    }

    pub fn process(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize) {
        self.data.process_messages();

        let mut i : usize = 0;
        for instrument in self.data.insruments.iter() {
            self.processors[i].set_current_preset_id(instrument.current_preset_id);
            i += 1;
        }

        if self.data.is_playing {
            self.update();
            for sender in self.audio_state_senders.iter() {
                sender.send(SequencerDataMessage::SetTick(self.data.tick)).unwrap();
            }
        }

        for s in 0..(nb_channels * num_samples) {
             outputs[s] = 0.0;
        }

        self.metronome.process(outputs, num_samples, nb_channels);
        
        for i in 0..self.processors.len() {
            self.processors[i].process(outputs, num_samples, nb_channels);
        }

        for s in 0..(nb_channels * num_samples) {
            outputs[s] *= self.data.volume;
        }
    }

    pub fn get_tick(&self) -> i32 {
        return self.data.tick;
    }

    pub fn note_on(&mut self, note_id: u8) {
        let idx = self.data.instrument_selected_id;
        if self.data.instrument_selected_id < self.processors.len() {
            self.processors[idx].note_on(note_id, 1.0);
            if self.data.is_recording {
                self.fifo_queue_midi_message.write(
                    MidiMessage {
                        first: NOTE_ON,
                        second: note_id,
                        third: 127,
                        tick: self.data.tick
                    }
                );
            }
        }
    }

    pub fn note_off(&mut self, note_id: u8) {
        let idx = self.data.instrument_selected_id;
        if self.data.instrument_selected_id < self.processors.len() {
            self.processors[idx].note_off(note_id);
                if self.data.is_recording {
                    self.fifo_queue_midi_message.write(
                        MidiMessage {
                            first: NOTE_OFF,
                            second: note_id,
                            third: 127,
                            tick: self.data.tick
                        }
                    );
                }
        }
    }

    pub fn clear_notes_events(&mut self, clear_all_instruments: bool) {
        for i in 0..self.processors.len() {
            if i == self.data.instrument_selected_id || clear_all_instruments {
                self.processors[i].clear_notes_events();
            }
        }
    }

}
