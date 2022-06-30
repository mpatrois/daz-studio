use std::ptr;

pub struct MetronomeVoice {
    pub active: bool,
    pub buffer_index: usize,
    pub buffer: *const Vec<f32>
}

impl MetronomeVoice {
    pub fn new() -> MetronomeVoice {
        MetronomeVoice {
            active: true,
            buffer_index: 0,
            buffer: ptr::null()
        }
    }
    pub fn render_next_block(&mut self, outputs: *mut f32, num_samples: usize, nb_channels: usize) {
        unsafe {
            let mut i = 0;
            while i < num_samples * nb_channels {
                if self.buffer_index < (*self.buffer).len() {
                    *outputs.offset(i as isize) += (*self.buffer)[self.buffer_index];
                    *outputs.offset((i + 1) as isize) += (*self.buffer)[self.buffer_index];
                } else {
                    self.active = false;
                }
                self.buffer_index += 1;
                i += 2;
            }
        }
    }

    pub fn is_ended(&self) -> bool {
        return !self.active;
    }
}