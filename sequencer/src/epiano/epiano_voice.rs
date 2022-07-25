#[derive(Copy, Clone)]
pub struct EpianoVoice {
    pub delta: i32, // sample playback
    pub frac: i32,
    pub pos: i32,
    pub end: i32,
    pub loop_idx: i32,

    pub env: f32, // envelope
    pub dec: f32,

    pub f0: f32,   // first-order LPF
    pub f1: f32,
    pub ff: f32,

    pub outl: f32,
    pub outr: f32,
    pub note: u8, // remember what note triggered this
    pub note_id: u8,
}

impl EpianoVoice {
    pub fn new() -> EpianoVoice {
        EpianoVoice {
            delta: 0,
            frac: 0,
            pos: 0,
            end: 0,
            loop_idx: 0,
            env: 0.,
            dec: 0.,
            f0: 0.,
            f1: 0.,
            ff: 0.,
            outl: 0.,
            outr: 0.,
            note: 0,
            note_id: 0,
        }
    }
}