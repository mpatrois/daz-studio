use crate::adsr::ADSR;
use crate::mood::mood_wave::MoodWave;

use std::rc::Rc;

#[derive(Clone)]
pub struct MoodOscillator {
    pub mood_wave: Rc<MoodWave>,
    pub pitch_ratio: f32,
    pub pitch_ratio_target: f32,
    pub position: f32,
    pub volume: f32,
    pub sample_rate: f32,
    pub glide: f32,
    pub glide_end: f32,
    pub glide_idx: f32,
    pub adsr: ADSR,
    pub phase_offset: f32
}

impl MoodOscillator {
    pub fn new(sample_rate: f32) -> MoodOscillator {
        MoodOscillator {
            sample_rate: sample_rate,
            mood_wave: Rc::new(MoodWave::empty()),
            pitch_ratio: 0.,
            pitch_ratio_target: 0.,
            position: 0.,
            volume: 0.,
            glide: 0.,
            glide_end: 0.,
            glide_idx: 0.,
            adsr: ADSR::new(0., 0., 1.0, 0., sample_rate),
            phase_offset: 0.0
        }
    }

    pub fn init(&mut self, frequency: f32, volume: f32, mood_wave: Rc<MoodWave>, adsr: ADSR, glide: f32, phase_offset: f32) {
        self.mood_wave = mood_wave;
        self.position = 0.;
        self.volume = volume;
        self.adsr = adsr;
        self.pitch_ratio = frequency / self.mood_wave.root_frequency;
        self.pitch_ratio_target = self.pitch_ratio;
        self.adsr.recalculate_rates();
        self.adsr.note_on();
        self.glide = glide;
        self.glide_end = glide * self.sample_rate; 
        self.glide_idx = 0.;

        self.position = self.pitch_ratio * (phase_offset * (1. / (frequency / self.sample_rate)));
    }

    pub fn set_target_pitch_ratio(&mut self, target_frequency: f32) {
        self.pitch_ratio_target = target_frequency / self.mood_wave.root_frequency;
        self.glide_idx = 0.;
        self.adsr.note_on();
    }

    pub fn tick(&mut self) -> f32 {
        let mut pitch_ratio = self.pitch_ratio;
        if self.pitch_ratio_target != self.pitch_ratio {
            let delta = (self.glide_idx / self.glide_end) * (self.pitch_ratio_target - self.pitch_ratio);
            pitch_ratio += delta;
            if self.glide_idx >= self.glide_end  {
                self.pitch_ratio = self.pitch_ratio_target;
            }
            self.glide_idx += 1.;
        }

        while self.position >= self.mood_wave.mood_wave_samples.len() as f32 {
            self.position -= self.mood_wave.mood_wave_samples.len() as f32;
        }

        let mut position = self.position;

        if self.phase_offset != 0.0 {
            position += self.phase_offset;
    
            while position < 0.0 {
                position += self.mood_wave.mood_wave_samples.len() as f32;
            }
    
            while position >= self.mood_wave.mood_wave_samples.len() as f32 {
                position -= self.mood_wave.mood_wave_samples.len() as f32;
            }
        }

       

        let pos = position as usize;
        let alpha = position - (pos as f32);
        let inv_alpha = 1.0 - alpha;

        let mut interpol_pos = pos + 1;
        if interpol_pos >= self.mood_wave.mood_wave_samples.len() {
            interpol_pos = interpol_pos - self.mood_wave.mood_wave_samples.len();
        }
        
        let s1 = self.mood_wave.mood_wave_samples[pos];
        let s2 = self.mood_wave.mood_wave_samples[interpol_pos];
        let s = s1 * inv_alpha + s2 * alpha;

        self.position += pitch_ratio;

        return s * self.volume * self.adsr.tick();
    }

    pub fn add_phase_offset(&mut self, offset: f32) {
        self.phase_offset = offset * self.mood_wave.mood_wave_samples.len() as f32;
    }
}