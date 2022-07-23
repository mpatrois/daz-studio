pub mod metronome;
pub mod oscillator;
pub mod synthesizer;
pub mod adsr;
pub mod decibels;
pub mod utils;
pub mod midimessage;
pub mod processor;
pub mod fifoqueue;
pub mod sampler;
pub mod mood;
pub mod noise;
pub mod preset;
pub mod fx;
pub mod sequencer_data;

use crate::processor::Processor;
use crate::mood::mood::Mood;
use crate::metronome::metronome::Metronome;
use crate::sampler::sampler::Sampler;
use crate::midimessage::NoteEvent;
use crate::sequencer_data::SequencerData;
use crate::sequencer_data::InstrumentData;
use crate::sequencer_data::Message as SequencerDataMessage;
use crate::midimessage::MidiMessage;
use crate::decibels::root_mean_square_stereo;

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
    nb_channels: usize,
    processors: Vec<Box<dyn Processor>>,
    processors_outputs: Vec<Vec<f32>>,
    pub audio_state_senders: Vec<Sender<sequencer_data::Message>>,
    has_new_notes: bool,
    stamp: i32,
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
            nb_channels: 2,
            processors: Vec::new(),
            processors_outputs: Vec::new(),
            data,
            audio_state_senders: Vec::new(),
            has_new_notes: false,
            stamp: 0,
        };

        sequencer.compute_elapsed_time_each_render();

        sequencer.add_processor(Box::new(Sampler::new(sample_rate, 0)));
        sequencer.add_processor(Box::new(Sampler::new(sample_rate, 1)));
        sequencer.add_processor(Box::new(Sampler::new(sample_rate, 2)));
        sequencer.add_processor(Box::new(Mood::new(sample_rate, 0)));
        sequencer.add_processor(Box::new(Mood::new(sample_rate, 1)));
        sequencer.add_processor(Box::new(Mood::new(sample_rate, 3)));
        sequencer.add_processor(Box::new(Mood::new(sample_rate, 4)));

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

    pub fn play_recorded_note_events(&mut self) {    
        for i in 0..self.processors.len() {
            for k in 0..self.processors[i].get_notes_events().len() {
                let note_event = self.processors[i].get_notes_events()[k];
                
                let record_recently = (self.stamp - note_event.stamp_record) < self.data.nb_ticks() / 2;
                if note_event.tick_off != -1 && (!record_recently || self.data.record_session != note_event.record_session) {
                    if note_event.tick_on == self.data.tick {
                        self.processors[i].note_on(note_event.note_id, 1.0);
                    }
                    if note_event.tick_off == self.data.tick {
                        self.processors[i].note_off(note_event.note_id);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.data.bpm_has_biped = false;
        self.time_accumulated += self.elapsed_time_each_render;
        while self.time_accumulated >= self.data.tick_time {
            self.time_accumulated -= self.data.tick_time;
          
            if !self.data.bpm_has_biped {
                self.data.bpm_has_biped = self.metronomome_tick();
            }

            self.play_recorded_note_events();

            self.data.tick += 1;
            self.stamp += 1;

            if self.data.tick >= self.data.nb_ticks() {
                self.data.tick = 0;
            }
        }
    }

    pub fn process(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize) {
        self.data.process_messages();

        if self.data.undo_last_session {
            if self.data.instrument_selected_id < self.processors.len() {
                let mut last_session = 0;
                let note_events = &mut self.processors[self.data.instrument_selected_id].get_notes_events();
                for note_event in note_events.iter() {
                    if note_event.record_session > last_session {
                        last_session = note_event.record_session;
                    }
                }

                while note_events.iter()
                    .position(|&n| n.record_session == last_session)
                    .map(|e| note_events.remove(e)).is_some() {}
                
                self.processors[self.data.instrument_selected_id].all_note_off();
            }
        }

        let mut i : usize = 0;
        for instrument in self.data.instruments.iter() {
            self.processors[i].set_current_preset_id(instrument.current_preset_id);
            i += 1;
        }

        let bpm_has_bipped = self.data.bpm_has_biped;
        if self.data.is_playing {
            self.update();
        }

        if self.data.kill_all_notes {
            for processor in self.processors.iter_mut() {
                processor.all_note_off();
            }
            self.data.kill_all_notes = false;
        }

        for s in 0..(nb_channels * num_samples) {
            outputs[s] = 0.0;
        }

        self.metronome.process(outputs, num_samples, nb_channels);
        
        for p_index in 0..self.processors.len() {
            let processor_outputs = &mut self.processors_outputs[p_index];
            for s in 0..processor_outputs.len() {
                processor_outputs[s] = 0.;
            }

            self.processors[p_index].process(processor_outputs, num_samples, nb_channels);
            let [rms_left, rms_right] = root_mean_square_stereo(processor_outputs, num_samples);
            self.data.instruments[p_index].rms_left = rms_left;
            self.data.instruments[p_index].rms_right = rms_right;
            for i in 0..processor_outputs.len() {
                outputs[i] += processor_outputs[i] * self.data.instruments[p_index].volume;
            }
        }

        for s in 0..(nb_channels * num_samples) {
            outputs[s] *= self.data.volume;
        }

        self.synchronise_data(bpm_has_bipped, outputs);
    }

    pub fn get_tick(&self) -> i32 {
        return self.data.tick;
    }

    pub fn note_on(&mut self, note_id: u8) {
        let idx = self.data.instrument_selected_id;
        if self.data.instrument_selected_id < self.processors.len() {
            self.processors[idx].note_on(note_id, 1.0);
            if self.data.is_recording && self.data.is_playing {
                let quantize_tick = self.quantize_tick();

                self.processors[idx].add_notes_event(NoteEvent {
                    tick_on: quantize_tick,
                    tick_off: -1,
                    note_id,
                    record_session: self.data.record_session,
                    stamp_record: self.stamp,
                });
                self.processors[idx].get_notes_events()
                    .sort_by(|a, b| a.tick_on.partial_cmp(&b.tick_on).unwrap());

                self.has_new_notes = true;
            }
        }
    }

    pub fn note_off(&mut self, note_id: u8) {
        let idx = self.data.instrument_selected_id;
        if self.data.instrument_selected_id < self.processors.len() {
            self.processors[idx].note_off(note_id);
                if self.data.is_recording && self.data.is_playing {
                
                    let quantize_tick = self.quantize_tick();

                    let note_events = self.processors[idx].get_notes_events();
                    for note_event in note_events.iter_mut() {
                        if note_event.note_id != note_id {
                            continue;
                        }
                        if note_event.tick_off != -1 {
                            continue;
                        }
                        note_event.tick_off = quantize_tick;
                      
                        if note_event.tick_off == note_event.tick_on {
                            note_event.tick_off += 120;
                            if note_event.tick_off > self.data.nb_ticks() - 1 {
                                note_event.tick_off = self.data.nb_ticks() - 1;
                            }
                        }
                        break;
                    }

                    self.has_new_notes = true;
                }
        }
    }

    pub fn add_processor(&mut self, mut processor: Box<dyn Processor>) {

        let nb_samples = self.buffer_size * self.nb_channels;
        let mut processor_outputs = Vec::with_capacity(self.buffer_size * self.nb_channels);

        for _i in 0..nb_samples {
            processor_outputs.push(0.);
        }

        processor.prepare(self.sample_rate, self.buffer_size, 2);
        
        self.data.instruments.push(InstrumentData {
            name: processor.get_name(),
            volume: 0.7,
            current_preset_id: processor.get_current_preset_id(),
            presets: processor.get_presets().iter().map(|preset| preset.get_name()).collect(),
            paired_notes: Vec::new(),
            rms_left: 0.,
            rms_right: 0.,
        });

        self.processors.push(processor);
        self.processors_outputs.push(processor_outputs);
    }

    fn quantize_tick(&self) -> i32 {
        if self.data.get_quantize() == -1 {
            if self.data.tick >= self.data.end_quantize_adjust() {
                return 0;
            }
            return self.data.tick;
        }
        let current_tick = self.data.tick;
        let interval = self.data.quantize_interval();
        let lower = current_tick / interval;
        let offset = current_tick % interval;
        let highest = offset / (interval / 2);

        let quantize_tick = (lower + highest) * interval;

        if quantize_tick >= self.data.end_quantize_adjust() {
            return 0;
        }
        return quantize_tick;
    }

    pub fn synchronise_data(&mut self, bpm_has_bipped: bool, _outputs: &mut [f32]) {
        for sender in self.audio_state_senders.iter() {
            if self.data.is_playing {
                sender.send(SequencerDataMessage::SetTick(self.data.tick)).unwrap();
                if !bpm_has_bipped && self.data.bpm_has_biped {
                    sender.send(SequencerDataMessage::SetBpmHasBiped(self.data.bpm_has_biped)).unwrap();
                }
            }
            if self.has_new_notes || self.data.undo_last_session {
                let idx = self.data.instrument_selected_id;
                if self.data.instrument_selected_id < self.processors.len() {
                    let note_events = self.processors[idx].get_notes_events().clone();
                    sender.send(SequencerDataMessage::SetMidiMessagesInstrument(note_events)).unwrap();
                }
                self.has_new_notes = false;
                self.data.undo_last_session = false;
            }

            for i in 0..self.data.instruments.len() {
                let rms_right = self.data.instruments[i].rms_right;
                let rms_left = self.data.instruments[i].rms_left;
                sender.send(SequencerDataMessage::SetRMSInstrument(i, rms_right, rms_left)).unwrap();
            }
        }
    }

}
