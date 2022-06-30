use crate::midimessage::MidiMessage;
use crate::sampler::sample::Sample;

pub trait Processor {
    fn note_on(&mut self, midi_note: u8, velocity: f32);
    fn note_off(&mut self, midi_note: u8);
    fn process(&mut self, outputs: *mut f32, num_samples: usize, nb_channels: usize);
    
    fn clear_notes_events(&mut self);
    fn get_notes_events(&mut self) -> &Vec<MidiMessage>;
    fn add_notes_event(&mut self, midi_message: MidiMessage);
    fn is_armed(&self) -> bool;
    fn set_is_armed(&mut self, is_armed: bool);

    fn get_id(&self) -> i32;

    fn add_sample(&mut self, sample: Sample);
}

// unsafe impl Send for Processor {

// } 