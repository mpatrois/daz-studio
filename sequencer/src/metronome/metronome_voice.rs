use std::ptr;

pub struct MetronomeVoice {
    pub active: bool,
    pub buffer_index: usize,
    // {todo} Raw pointers are almost never used in Rust. `buffer`
    // could be a reference but you would have to deal with
    // lifetimes. A safe and easy to use version of `*const Vec<f32>`
    // is `Option<Rc<Vec<f32>>>`.
    pub buffer: *const Vec<f32>
}

impl MetronomeVoice {
    pub fn new() -> MetronomeVoice {
        MetronomeVoice {
            active: true,
            buffer_index: 0,
            // {todo} See above comment. `buffer` would be initialized
            // to `None`.
            buffer: ptr::null()
        }
    }
    pub fn render_next_block(&mut self, outputs: *mut f32, num_samples: usize, nb_channels: usize) {
        // {todo} See other comments about `unsafe` usage.
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