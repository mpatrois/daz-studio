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

        let mut left_channel: Vec<f32> = Vec::with_capacity(samples.len());
        let mut right_channel: Vec<f32> = Vec::with_capacity(samples.len());

        if header.channels == 1 {
            left_channel = samples.clone();
            right_channel = samples.clone();
        } else if header.channels == 2 {
            let mut i = 0;
            while i < samples.len() {
                left_channel.push(samples[i]);
                right_channel.push(samples[i+1]);
                i += 2;
            }
        }
    
        return Sample {
            sample_rate: sample_rate,
            size: left_channel.len(),
            left_channel: left_channel,
            right_channel: right_channel,
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