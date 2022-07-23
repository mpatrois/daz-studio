const MAX_NOTES : usize = 8;

use crate::mood::mood_voice::MoodVoice;
use crate::mood::mood_wave_bank::MoodWaveBank;
use crate::mood::mood_preset::MoodPreset;
use crate::mood::mood_preset_bank::get_mood_presets;

use crate::processor::Processor;
use crate::midimessage::NoteEvent;
use crate::preset::Preset;
use crate::fx::reverb::Reverb;

use std::rc::Rc;

pub struct Mood {
    pub wave_bank: Rc<MoodWaveBank>,
    pub note_events: Vec<NoteEvent>,
    voices: Vec<MoodVoice>,
    nb_actives_notes: usize,
    presets: Vec<MoodPreset>,
    preset_id: usize,
    reverb: Reverb
}

impl Mood {
    pub fn new(sample_rate: f32, id: usize) -> Mood {

        let mut voices : Vec<MoodVoice> = Vec::new();

        let wave_bank = Rc::new(MoodWaveBank::new(sample_rate));

        for _i in 0..MAX_NOTES {
            voices.push(MoodVoice::new(sample_rate, wave_bank.clone()))
        }

        Mood {
            wave_bank,
            note_events: Vec::new(),
            voices: voices,
            nb_actives_notes: 0,
            presets: get_mood_presets(),
            preset_id: id,
            reverb: Reverb::new(sample_rate),
        }
    }
}

impl Processor for Mood {

    fn get_name(&self) -> String { "Mood".to_string() }

    fn note_on(&mut self, midi_note: u8, velocity: f32) {
        if self.presets[self.preset_id].is_mono {
            if self.voices[0].active {
                self.voices[0].set_target_note(midi_note);
            } else {
                self.voices[self.nb_actives_notes].start_note(midi_note, velocity, self.presets[self.preset_id].clone());
                self.nb_actives_notes += 1;
            }
        } else {
            if self.nb_actives_notes < MAX_NOTES - 1 {
                self.voices[self.nb_actives_notes].start_note(midi_note, velocity, self.presets[self.preset_id].clone());
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

        for s in outputs.iter_mut() {
            *s = 0.0;
        }

        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if self.voices[i].active {
                self.voices[i].render_next_block(outputs, nb_channels);
            }
        }
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if !self.voices[i].active {
                self.nb_actives_notes -= 1;
                let active_notes = self.nb_actives_notes as usize;
                self.voices[i] = self.voices[active_notes].clone();
            }
        }

        if self.presets[self.preset_id].reverb_enabled {
            self.reverb.process(outputs, num_samples);
        }

        // for i in 0..self.mood_outputs.len() {
        //     outputs[i] += self.mood_outputs[i];
        // }
    }

    fn get_notes_events(&mut self) -> &mut Vec<NoteEvent> {
        return &mut self.note_events;
    }

    fn add_notes_event(&mut self, midi_message: NoteEvent) {
        self.note_events.push(midi_message);
    }

    fn get_current_preset_id(&self) -> usize {
        self.preset_id
    }

    fn set_current_preset_id(&mut self, id: usize) {
        if self.preset_id != id {
            self.preset_id = id;
            self.reverb.set_parameters(self.presets[id].reverb_params.clone());
            self.reverb.reset();
        }
    }

    fn get_presets(&self) -> Vec<Box<dyn Preset>> {
        let mut presets : Vec<Box<dyn Preset>> = Vec::new();
        for preset in &self.presets {
            presets.push(Box::new(preset.clone()));
        }
        return presets;
    }

    fn prepare(&mut self, _sample_rate: f32, _num_samples: usize, _nb_channels: usize) {}
}