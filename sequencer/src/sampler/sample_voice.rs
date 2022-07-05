use std::ptr;

use crate::adsr::ADSR;
use crate::sampler::sample::Sample;

#[derive(Copy, Clone)]
pub struct SamplerVoice {
    // {todo} Try to avoid using `pub` everywhere, but if it's an internal
    // `struct` it might be OK.
    pub active: bool,
    pub sample: *const Sample,
    pub velocity: f32,
    pub pitch_ratio: f32,
    pub sample_rate: f32,
    pub adsr: ADSR,
    pub root_midi_note: i8,
    pub root_note_min: i8,
    pub root_note_max: i8,
    pub loop_crossfade_duration: usize,
    pub loop_length: usize,
    pub note_id: u8,
    pub source_sample_position: f32
}

impl SamplerVoice {
    pub fn new(sample_rate: f32) -> SamplerVoice {
        SamplerVoice {
            active: true,
            sample: ptr::null(),
            velocity: 1.0,
            pitch_ratio: 0.0,
            sample_rate: sample_rate,
            adsr: ADSR::new(0.0001, 0.0, 1.0, 0.4, sample_rate),
            loop_crossfade_duration: 0,
            loop_length: 0,
            root_midi_note: 60,
            root_note_min: 60,
            root_note_max: 60,
            note_id: 0,
            source_sample_position: 0.0
        }
    }

    pub fn start_note(&mut self, midi_note :u8, velocity: f32, sample: *const Sample) {
        self.sample = sample;
        self.note_id = midi_note;
    
        // {todo} Better wrap `self.sample` in an `Option` and use `if let`.
        if self.sample.is_null() { return };

        self.active = true;

        self.adsr.note_on();
        
        // {todo} The use of `unsafe` should be avoided or wrapped inside
        // a dedicated type with careful checks.
        unsafe {
            self.adsr.set_sample_rate((*self.sample).sample_rate);
            let midi_delta = ((midi_note - (*self.sample).root_midi_note)) as f32 / 12.0;
            self.pitch_ratio = f32::powf(2.0, midi_delta as f32) * (*self.sample).sample_rate / self.sample_rate;
            self.source_sample_position = 0.0;
        }

        self.velocity = velocity;
    }

    pub fn stop_note(&mut self) {
        self.adsr.note_off();
    }

    pub fn render_next_block(&mut self, outputs: *mut f32, num_samples: usize) {
        if self.sample.is_null() { return };
        
        // {todo} The use of `unsafe` should be avoided or wrapped inside
        // a dedicated type with careful checks.
        unsafe {
            let output_left = outputs;
            let output_right = (outputs).offset(num_samples as isize);
            for i in 0..num_samples {
                let envelope_value = self.adsr.tick();

                let pos = self.source_sample_position as usize;
                let alpha = self.source_sample_position - (pos as f32);
                let inv_alpha = 1.0 - alpha;

                if envelope_value == 0.0 {
                    self.adsr.reset();
                    self.active = false;
                    break;
                }

                if pos < (*self.sample).size {
                    let volume = envelope_value * self.velocity;
                    let mut interpol_pos = pos + 1;
                    if interpol_pos >= (*self.sample).size {
                        interpol_pos = pos;
                    }
                    let left = (*self.sample).left_channel[pos] * inv_alpha + (*self.sample).left_channel[interpol_pos] * alpha;
                    let right = (*self.sample).right_channel[pos] * inv_alpha + (*self.sample).right_channel[interpol_pos] * alpha;
                    *output_left.offset(i as isize) += left * volume;
                    *output_right.offset(i as isize) += right * volume;
                    self.source_sample_position += self.pitch_ratio;
                } else {
                    self.active = false;
                    self.adsr.reset();
                    break;
                }
            }
        }
    }

    pub fn is_ended(&self) -> bool {
        // {todo} The `return` keyword is useless here.
        return !self.active;
    }
}