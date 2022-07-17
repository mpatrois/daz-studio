use crate::midimessage::NoteEvent;
use crate::processor::Processor;
use crate::synthesizer::synthesizervoice::SynthesizerVoice;

use crate::synthesizer::operator::SINE;
use crate::synthesizer::operator::SAW_ANALOGIC_64;
use crate::synthesizer::operator::SAW_ANALOGIC_4;
use crate::synthesizer::operator::SAW_DIGITAL;
use crate::synthesizer::operator::OSC_OFF;
use crate::synthesizer::synthesizer_preset::SynthesizerPreset;
use crate::preset::Preset;

use crate::decibels::db_to_gain;

use biquad::*;
use biquad::Type;

const MAX_VOICES : usize = 1;

pub struct Synthesizer {
    voices: Vec<SynthesizerVoice>,
    nb_actives_notes: usize,
    pub id: usize,
    pub note_events: Vec<NoteEvent>,
    im_armed: bool,
    sample_rate: f32,
    presets: Vec<SynthesizerPreset>,
    preset_id: usize,
}

impl Synthesizer {
    pub fn new(sample_rate: f32, id: usize, preset_id: usize) -> Synthesizer {

        let mut voices : Vec<SynthesizerVoice> = Vec::new();

        for _i in 0..MAX_VOICES {
            voices.push(SynthesizerVoice::new(sample_rate))
        }

        let mut synth = Synthesizer {
            voices: voices,
            nb_actives_notes: 0,
            id: id,
            note_events: Vec::with_capacity(100),
            im_armed: false,
            sample_rate: sample_rate,
            presets: Vec::new(),
            preset_id: preset_id
        };

        synth.presets.push(SynthesizerPreset {
            id: 0,
            name: "Guitar bass".to_string(),
            algorithm: 5,
            nb_voices: 1,
            filter_type: Type::LowPass,
            filter_f0: 880.hz() as biquad::Hertz<f32>,
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            
            oscx_coarse: [0.5, 0.5, 1.0, 0.5],
            oscx_level: [db_to_gain(-25.), db_to_gain(-20.), db_to_gain(-12.0), db_to_gain(0.0)],
            oscx_osc_type: [SINE, SINE, SINE, SAW_ANALOGIC_4],
            oscx_phase_offset: [0.0, 0.7, 0., db_to_gain(0.0)],
            oscx_feedback: [0.4, 0., 0., 0.],
            oscx_adsr_attack: [0.0128, 0.00092, 0.00423, 0.00243],
            oscx_adsr_decay: [3.38, 0.969, 60.0, 2.33],
            oscx_adsr_sustain: [db_to_gain(-100.), db_to_gain(-100.), db_to_gain(-33.), db_to_gain(-11.)],
            oscx_adsr_release: [0.05, 6.26, 0.145, 0.05],
        });
        
        synth.presets.push(SynthesizerPreset {
            id: 1,
            name: "G-FUNK bass".to_string(),
            algorithm: 11,
            nb_voices: 1,
            filter_type: Type::LowPass,
            filter_f0: (880 * 2).hz() as biquad::Hertz<f32>,
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            
            oscx_coarse: [0.5, 0.5, 1.0, 0.5],
            oscx_level: [db_to_gain(-100.), db_to_gain(-100.), db_to_gain(-100.0), db_to_gain(0.0)],
            oscx_osc_type: [OSC_OFF, OSC_OFF, OSC_OFF, SAW_ANALOGIC_64],
            oscx_phase_offset: [0.0, 0.7, 0., 0.],
            oscx_feedback: [0.4, 0., 0., 0.],
            oscx_adsr_attack: [0.0128, 0.00092, 0.00423, 0.00243],
            oscx_adsr_decay: [3.38, 0.969, 60.0, 0.],
            oscx_adsr_sustain: [db_to_gain(-100.), db_to_gain(-100.), db_to_gain(-33.), db_to_gain(-11.)],
            oscx_adsr_release: [0.05, 0.2, 0.145, 0.05],
        });

        synth.presets.push(SynthesizerPreset {
            id: 2,
            name: "G-FUNK lead".to_string(),
            algorithm: 11,
            nb_voices: 1,
            filter_type: Type::BandPass,
            filter_f0: 10.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,

            oscx_coarse: [0.5, 0.5, 3.98, 4.],
            oscx_level: [db_to_gain(-100.), db_to_gain(-100.), db_to_gain(-1.0), db_to_gain(-1.0)],
            oscx_osc_type: [OSC_OFF, OSC_OFF, OSC_OFF, SAW_ANALOGIC_64],
            oscx_phase_offset: [0.0, 0.0, 0., 0.],
            oscx_feedback: [0.0, 0., 0., 0.],
            oscx_adsr_attack: [0.0128, 0.00092, 0.00243, 0.00243],
            oscx_adsr_decay: [3.38, 0.969, 0., 0.],
            oscx_adsr_sustain: [db_to_gain(-100.), db_to_gain(-100.), db_to_gain(-100.), db_to_gain(-2.)],
            oscx_adsr_release: [0.05, 0.2, 0.05, 0.05],
        });
        
        return synth;
    }
}


impl Processor for Synthesizer {

    fn get_name(&self) -> String { "Synthesizer".to_string() }

    fn note_on(&mut self, midi_note: u8, velocity: f32) {

        let frequency = f32::powf(2.0, (((midi_note) as i32 - 69) as f32 / 12.0) as f32) * 440.0;

        if self.voices.len() == 1 && self.voices[0].active {
            // println!("here");
            self.voices[0].note_id = midi_note;
            self.voices[0].target_frequency = frequency;
            // self.voices[0].target_frequency_step = (self.voices[0].target_frequency - self.voices[0].frequency) / 1000.;
            // println!("{} {} {}", self.voices[0].frequency, self.voices[0].target_frequency, self.voices[0].target_frequency_step);
            for i in 0..self.voices[0].operators.len() {
            //     self.voices[0].operators[i].adsr.reset();
                self.voices[0].operators[i].adsr.note_on();
            }
        } else {
            if self.nb_actives_notes < MAX_VOICES {
                let note_to_active = self.nb_actives_notes as usize;

                let preset = self.presets[self.preset_id].clone();
                
                self.voices[note_to_active].algorithm = preset.algorithm;
    
                self.voices[note_to_active].filter_type = preset.filter_type;
                self.voices[note_to_active].filter_f0 = preset.filter_f0;
                self.voices[note_to_active].filter_q_value = preset.filter_q_value;
                
                self.voices[note_to_active].frequency = frequency;
                self.voices[note_to_active].target_frequency = frequency;
    
                for operator_idx in 0..self.voices[note_to_active].operators.len() {
                    self.voices[note_to_active].target_frequency_step = 0.;
                    self.voices[note_to_active].operators[operator_idx].init(
                        self.sample_rate, 
                        preset.oscx_coarse[operator_idx] * frequency, 
                        preset.oscx_level[operator_idx], 
                        preset.oscx_osc_type[operator_idx],
                        preset.oscx_phase_offset[operator_idx], 
                        preset.oscx_feedback[operator_idx],
                    );
                    self.voices[note_to_active].operators[operator_idx].adsr.set_adsr(
                        preset.oscx_adsr_attack[operator_idx],
                        preset.oscx_adsr_decay[operator_idx],
                        preset.oscx_adsr_sustain[operator_idx],
                        preset.oscx_adsr_release[operator_idx],
                    );
                }
                
                self.voices[note_to_active].start_note(midi_note, velocity);
                self.nb_actives_notes += 1;
            }
        }

    }

    fn note_off(&mut self, midi_note: u8) {
        for i in 0..self.voices.len() {
            if self.voices[i].note_id == midi_note && self.voices[i].active {
                self.voices[i].stop_note();
            }
        }
    }

    fn all_note_off(&mut self) {
        for i in 0..self.voices.len() {
            self.voices[i].stop_note();
        }
    }

    fn process(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize) {
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if !self.voices[i].is_ended() {
                self.voices[i].render_next_block(outputs, num_samples, nb_channels);
            }
        }
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if self.voices[i].is_ended() {
                self.nb_actives_notes -= 1;
                let active_notes = self.nb_actives_notes as usize;
                self.voices[i] = self.voices[active_notes];
            }
        }
    }

    fn clear_notes_events(&mut self) {
        for i in 0..self.nb_actives_notes {
            self.voices[i].stop_note();
        }
        self.note_events.clear();
    }

    fn get_notes_events(&mut self) -> &mut Vec<NoteEvent> {
        return &mut self.note_events;
    }

    fn add_notes_event(&mut self, midi_message: NoteEvent) {
        self.note_events.push(midi_message);
    }

    fn is_armed(&self) -> bool {
        return self.im_armed;
    }

    fn set_is_armed(&mut self, is_armed: bool) {
        self.im_armed = is_armed;
    }

    fn get_id(&self) -> usize {
        return self.id;
    }

    fn get_current_preset_id(&self) -> usize {
        self.preset_id
    }

    fn set_current_preset_id(&mut self, id: usize) {
        self.preset_id = id;
    }

    fn get_presets(&self) -> Vec<Box<dyn Preset>> {
        let mut presets : Vec<Box<dyn Preset>> = Vec::new();
        for preset in &self.presets {
            presets.push(Box::new(preset.clone()));
        }
        return presets;
    }
}