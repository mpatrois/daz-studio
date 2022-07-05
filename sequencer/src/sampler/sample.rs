use std::fs::File;
use std::path::Path;
use crate::sampler::sampler_preset::SampleInfo;
use wav_io::{reader, writer, utils, resample, splitter, header::*, tone};

pub struct Sample {
    pub sample_rate: f32,
    pub left_channel: Vec<f32>,
    pub right_channel: Vec<f32>,
    pub size: usize,
    pub root_midi_note: u8,
    pub note_midi_min: u8,
    pub note_midi_max: u8,
}

impl Sample {
    pub fn load_sample(sample_info: &SampleInfo, sample_rate: f32) -> Sample {

        let file_in = File::open(sample_info.filepath.clone()).unwrap();
        let (header, samples) = wav_io::read_from_file(file_in).unwrap();

        let samples2 = resample::linear(samples, header.channels, header.sample_rate, sample_rate as u32);
        
        // let mut left_channel : Vec<f32> = Vec::new();
        // let mut right_channel : Vec<f32> = Vec::new();

    
        println!("={:?}", header);
        
        return Sample {
            sample_rate: sample_rate,
            size: samples2.len(),
            left_channel: samples2.clone(),
            right_channel: samples2.clone(),
            root_midi_note: sample_info.root_midi_note,
            note_midi_min: sample_info.note_midi_min,
            note_midi_max: sample_info.note_midi_max,
        }
    }

    pub fn apply_to_note(&self, midi_note: u8) -> bool {
        if self.note_midi_min <= midi_note && midi_note <= self.note_midi_max  {
            return true;
        }
        return false;
    } 
}