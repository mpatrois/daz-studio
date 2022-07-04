use crate::midimessage::MidiMessage;
use crate::sampler::sample::Sample;
use crate::preset::Preset;

pub trait Processor {
    fn note_on(&mut self, midi_note: u8, velocity: f32);
    fn note_off(&mut self, midi_note: u8);
    fn process(&mut self, outputs: *mut f32, num_samples: usize, nb_channels: usize);
    
    fn clear_notes_events(&mut self);
    fn get_notes_events(&mut self) -> &Vec<MidiMessage>;
    fn add_notes_event(&mut self, midi_message: MidiMessage);
    fn is_armed(&self) -> bool;
    fn set_is_armed(&mut self, is_armed: bool);

    fn get_id(&self) -> usize;

    fn get_current_preset(&self) -> Box<dyn Preset>;
    fn get_presets(&self) -> Vec<Box<dyn Preset>>;
    fn next_presets(&self);
    fn previous_presets(&self);

    fn add_sample(&mut self, sample: Sample);
}

// unsafe impl Send for Processor {

// } 