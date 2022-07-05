
const NB_OPERATORS : usize = 4;

use crate::preset::Preset;
use std::fs::File;
use std::io::Read;


#[derive(Clone)]
pub struct SampleInfo {
    pub root_midi_note: u8,
    pub note_midi_min: u8,
    pub note_midi_max: u8,
    pub filepath: String,
}

#[derive(Clone)]
pub struct SamplerPreset {
    pub id: usize,
    pub name: String,
    pub samples: Vec<SampleInfo>,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl Preset for SamplerPreset {
    fn get_id(self) -> usize {
        return self.id;
    }

    fn get_name(self) -> String {
        return self.name;
    }
}