use crate::midimessage::NoteEvent;
use crate::preset::Preset;

pub trait Processor {
    fn note_on(&mut self, midi_note: u8, velocity: f32);
    fn note_off(&mut self, midi_note: u8);
    fn all_note_off(&mut self);
    fn process(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize);
    fn prepare(&mut self, sample_rate: f32, num_samples: usize, nb_channels: usize);
    
    fn get_notes_events(&mut self) -> &mut Vec<NoteEvent>;
    fn add_notes_event(&mut self, midi_message: NoteEvent);

    fn get_name(&self) -> String;

    fn set_current_preset_id(&mut self, id: usize);
    fn get_current_preset_id(&self) -> usize;
    fn get_presets(&self) -> Vec<Box<dyn Preset>>;
}