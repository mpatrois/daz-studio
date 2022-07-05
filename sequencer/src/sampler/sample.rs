
// {todo} Using `pub` everywhere is not good for encapsulation, but
// why not if it's an internal `struct`.
pub struct Sample {
    pub sample_rate: f32,
    // pub midi_note: i8,
    pub left_channel: Vec<f32>,
    pub right_channel: Vec<f32>,
    pub size: usize,
    pub root_midi_note: u8,
    pub note_midi_min: u8,
    pub note_midi_max: u8,
    pub loop_crossfade_duration: usize,
    pub loop_length: usize
}

impl Sample {
    pub fn new(array_ptr: *const f32, data_size: usize, sample_rate: f32) -> Sample {
        let length_channel = data_size / 2;
        let mut left_channel : Vec<f32> = Vec::with_capacity(data_size / 2);
        let mut right_channel : Vec<f32> = Vec::with_capacity(data_size / 2);
        for i in 0..length_channel {
            // {todo} See other comments about `unsafe` usage.
            unsafe {
                left_channel.push(*array_ptr.offset((i) as isize));
                right_channel.push(*array_ptr.offset((length_channel + i) as isize))
            }
        }
        // {todo} The `return` keyword is useless here.
        return Sample {
            sample_rate: sample_rate,
            // midi_note: 67,
            left_channel: left_channel,
            right_channel: right_channel,
            size: length_channel,
            root_midi_note: 60,
            note_midi_min: 0,
            note_midi_max: 0,
            loop_crossfade_duration: 0,
            loop_length: 0,
        }
    }

    pub fn apply_to_note(&self, midi_note: u8) -> bool {
        if self.note_midi_min <= midi_note && midi_note <= self.note_midi_max  {
            // {todo} This `return` could easily be avoided with an
            // `else` branch.
            return true;
        }
        // {todo} `return` useless
        return false;
    } 
}