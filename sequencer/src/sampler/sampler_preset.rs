use crate::preset::Preset;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
// use std::io::Read;
// use std::path::Path;

use serde::{Deserialize};

#[derive(Clone, Deserialize)]
pub struct SampleInfo {
    pub root_midi_note: u8,
    pub note_midi_min: u8,
    pub note_midi_max: u8,
    pub filepath: String,
    pub is_one_shot: bool,
}

#[derive(Clone, Deserialize)]
pub struct SamplerPreset {
    pub id: usize,
    pub name: String,
    pub samples: Vec<SampleInfo>,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl SamplerPreset {
    pub fn new(filepath: String) -> Result<SamplerPreset, Box<dyn Error>>  {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        let sampler_preset: SamplerPreset = serde_json::from_reader(reader)?;
        Ok(sampler_preset)
    }
}

impl Preset for SamplerPreset {
    fn get_id(self) -> usize {
        return self.id;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}