use crate::adsr::ADSR;
use crate::sampler::sample::Sample;
use std::rc::Rc;

#[derive(Clone)]
pub struct SamplerVoice {
    pub active: bool,
    pub sample: Rc<Sample>,
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
            sample: Rc::new(Sample::empty()),
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

    pub fn start_note(&mut self, midi_note :u8, velocity: f32, sample: Rc<Sample>) {
        self.sample = sample;
        self.note_id = midi_note;

        self.active = true;

        self.adsr.note_on();
        

        self.adsr.set_sample_rate((*self.sample).sample_rate);
        let midi_delta = ((midi_note - (*self.sample).root_midi_note)) as f32 / 12.0;
        self.pitch_ratio = f32::powf(2.0, midi_delta as f32) * (*self.sample).sample_rate / self.sample_rate;
        self.source_sample_position = 0.0;

        self.velocity = velocity;
    }

    pub fn stop_note(&mut self) {
        if !self.sample.is_one_shot {
            self.adsr.note_off();
        }
    }

    pub fn render_next_block(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize) {
        
        let mut idx = 0;
        while idx < nb_channels * num_samples {
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

                outputs[idx] += left * volume;
                outputs[idx + 1] += right * volume;

                self.source_sample_position += self.pitch_ratio;
            } else {
                self.active = false;
                self.adsr.reset();
                break;
            }
            idx += 2;
        }
    }

    pub fn is_ended(&self) -> bool {
        return !self.active;
    }
}