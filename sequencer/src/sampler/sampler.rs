use crate::processor::Processor;
use crate::midimessage::NoteEvent;
use crate::sampler::sample::Sample;
use crate::sampler::sample_voice::SamplerVoice;
use crate::sampler::sampler_preset::SamplerPreset;
use crate::preset::Preset;
use std::rc::Rc;

const MAX_NOTES : usize = 32;

pub struct Sampler {
    pub samples: Vec<Rc<Sample>>,
    voices: Vec<SamplerVoice>,
    nb_actives_notes: usize,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub id: usize,
    pub note_events: Vec<NoteEvent>,
    pub im_armed: bool,
    presets: Vec<SamplerPreset>,
    preset_id: usize,
    sample_rate: f32,
}

impl Sampler {
    pub fn new(sample_rate: f32, preset_id: usize) -> Sampler {

        let mut voices : Vec<SamplerVoice> = Vec::new();

        for _i in 0..MAX_NOTES {
            voices.push(SamplerVoice::new(sample_rate))
        }

        let mut sampler = Sampler {
            sample_rate: sample_rate,
            voices: voices,
            nb_actives_notes: 0,
            samples: Vec::new(),
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
            id: 0,
            note_events: Vec::with_capacity(100),
            im_armed: false,
            presets: Vec::new(),
            preset_id: preset_id,
        };

        let presets = [
            "./data/sampler-presets/Daz-Funk/preset.json",
            "./data/sampler-presets/Daz-Kit/preset.json",
            "./data/sampler-presets/Daz-Special/preset.json",
        ];

        for preset in presets {
            let sampler_preset = SamplerPreset::new(preset.to_string());
            sampler.presets.push(sampler_preset.unwrap());
        }

        sampler.load_samples();

        return sampler;
    }

    fn load_samples(&mut self) {
        self.all_note_off();
        self.samples.clear();
        for sample_info in self.presets[self.preset_id].samples.iter() {
            let sample = Sample::load_sample(sample_info, self.sample_rate);
            self.samples.push(Rc::new(sample));
        }


    }
}

impl Processor for Sampler {

    fn get_name(&self) -> String { "Sampler".to_string() }

    fn note_on(&mut self, midi_note: u8, velocity: f32) {
        for sample_idx in 0..self.samples.len() {
            if self.samples[sample_idx].apply_to_note(midi_note) {
                if self.nb_actives_notes < MAX_NOTES - 1 {
                    let note_to_active = self.nb_actives_notes as usize;
                    self.voices[note_to_active].adsr.attack = self.attack;
                    self.voices[note_to_active].adsr.decay = self.decay;
                    self.voices[note_to_active].adsr.sustain = self.sustain;
                    self.voices[note_to_active].adsr.release = self.release;
                    self.voices[note_to_active].adsr.reset();
                    self.voices[note_to_active].adsr.recalculate_rates();

                    self.voices[note_to_active].start_note(midi_note, velocity, self.samples[sample_idx].clone());
                    self.nb_actives_notes += 1;
                    break;
                }
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
                self.voices[i] = self.voices[active_notes].clone();
            }
        }
    }

    // fn get_notes_events(&mut self) -> &mut Vec<NoteEvent> {
    //     return &mut self.note_events;
    // }

    // fn add_notes_event(&mut self, midi_message: NoteEvent) {
    //     self.note_events.push(midi_message);
    // }

    fn get_current_preset_id(&self) -> usize {
        self.preset_id
    }

    fn set_current_preset_id(&mut self, id: usize) {
        if self.preset_id != id {
            self.preset_id = id;
            self.load_samples();
        }
    }

    fn get_presets(&self) -> Vec<Box<dyn Preset>> {
        let mut presets : Vec<Box<dyn Preset>> = Vec::new();
        for preset in &self.presets {
            presets.push(Box::new(preset.clone()));
        }
        return presets;
    }

    fn prepare(&mut self, _sample_rate: f32, _num_samples: usize, _nb_channels: usize) {
        
    }
    
}