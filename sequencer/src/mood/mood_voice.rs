
use crate::adsr::ADSR;
use crate::mood::mood_wave_bank::MoodWaveBank;
use crate::mood::mood_preset::MoodPreset;
use crate::mood::mood_oscillator::MoodOscillator;
use crate::utils::midi_note_fo_hertz;

use std::rc::Rc;

const NB_OSCILLATORS : usize = 3;

pub const OSC_1 : usize = 0;
pub const OSC_2 : usize = 1;
pub const OSC_3 : usize = 2;

use biquad::*;
use biquad::Type;
use biquad::DirectForm1;

#[derive(Clone)]
pub struct MoodVoice {
    pub wave_bank: Rc<MoodWaveBank>,
    pub active: bool,
    pub note_id: u8,
    pub sample_rate: f32,
    pub oscx: [MoodOscillator; NB_OSCILLATORS],
    pub biquad_filter: DirectForm1::<f32>,
    pub filter_type: biquad::Type<f32>,
    pub filter_f0: biquad::Hertz<f32>,
    pub filter_q_value: f32,
    pub mood_preset: MoodPreset,
}

impl MoodVoice {

    pub fn new(sample_rate: f32, wave_bank: Rc<MoodWaveBank>) ->  MoodVoice {

        let f0 = 880.hz();
        let fs = 30.khz();

        let coeffs = Coefficients::<f32>::from_params(Type::AllPass, fs, f0, biquad::Q_BUTTERWORTH_F32).unwrap();
        let biquad_filter = DirectForm1::<f32>::new(coeffs);

        MoodVoice {
            sample_rate: sample_rate,
            wave_bank: wave_bank.clone(),
            active: false,
            note_id: 0,
            oscx: [
                MoodOscillator::new(sample_rate),
                MoodOscillator::new(sample_rate),
                MoodOscillator::new(sample_rate),
            ],
            biquad_filter: biquad_filter,
            filter_type: Type::LowPass,
            filter_f0: f0,
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            mood_preset: MoodPreset::empty()
        }
    }

    pub fn start_note(&mut self, midi_note :u8, _velocity: f32, mood_preset: MoodPreset) {
        self.note_id = midi_note;
        self.active = true;
        self.mood_preset = mood_preset;

        let pitch = midi_note_fo_hertz(midi_note);

        for i in 0..NB_OSCILLATORS {
            let adsr = ADSR::new(
                self.mood_preset.oscx_adsr_attack[i], 
                self.mood_preset.oscx_adsr_decay[i], 
                self.mood_preset.oscx_adsr_sustain[i], 
                self.mood_preset.oscx_adsr_release[i], 
                self.sample_rate
            );

            let twelve_root_of_two = f64::powf(2., 1./12.);
            let range = 8. / self.mood_preset.oscx_octave_range[i];
            let frequency = (pitch * range) * f64::powf(twelve_root_of_two, self.mood_preset.oscx_semitone_shift[i] as f64) as f32;
        
            let wave_result = self.wave_bank.find_mood_wave(frequency, self.mood_preset.oscx_wave_form[i].clone());

            if wave_result.is_ok() {
                self.oscx[i].init(
                    frequency,
                    self.mood_preset.oscx_volume[i],
                    wave_result.unwrap(),
                    adsr,
                    self.mood_preset.glide,
                    self.mood_preset.oscx_phase_offset[i],
                );
            }
        }

        self.filter_type = self.mood_preset.filter_type;
        self.filter_f0 = self.mood_preset.filter_f0;
        self.filter_q_value = self.mood_preset.filter_q_value;

        let coeffs = Coefficients::<f32>::from_params(self.filter_type, self.sample_rate.hz(), self.filter_f0, self.filter_q_value).unwrap();
        self.biquad_filter = DirectForm1::<f32>::new(coeffs);
        self.biquad_filter.reset_state();
    }

    pub fn set_target_note(&mut self, target_note: u8) {

        self.note_id = target_note;
        let pitch = midi_note_fo_hertz(target_note);

        for i in 0..NB_OSCILLATORS {

            let twelve_root_of_two = f64::powf(2., 1./12.);
            let range = 8. / self.mood_preset.oscx_octave_range[i];
            let target_frequency = (pitch * range) * f64::powf(twelve_root_of_two, self.mood_preset.oscx_semitone_shift[i] as f64) as f32;
        
            self.oscx[i].set_target_pitch_ratio(target_frequency);
            self.oscx[i].set_target_pitch_ratio(target_frequency);
            self.oscx[i].set_target_pitch_ratio(target_frequency);
        }

    }

    pub fn stop_note (&mut self) {
        self.oscx[OSC_1].adsr.note_off();
        self.oscx[OSC_2].adsr.note_off();
        self.oscx[OSC_3].adsr.note_off();
    }

    pub fn render_next_block(&mut self, outputs: &mut [f32], _nb_channels: usize) {

        let mut idx = 0;

        let mut out = 0.0;
        while idx < outputs.len() {
            
            out = self.oscx[OSC_1].tick() + self.oscx[OSC_2].tick() + self.oscx[OSC_3].tick();
            out = self.biquad_filter.run(out);
            outputs[idx] += out;
            outputs[idx + 1] += out;

            
            idx += 2;
        }
        if out.abs() <= 0.000000000001 {
            
            self.active = false;
        }
    }
}