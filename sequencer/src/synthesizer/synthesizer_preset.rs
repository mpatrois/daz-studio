
const NB_OPERATORS : usize = 4;


// use biquad;

#[derive(Clone)]
pub struct SynthesizerPreset {
    pub name: String,
    pub algorithm: u8,
    pub nb_voices: usize, 
    pub filter_type: biquad::Type<f32>,
    pub filter_f0: biquad::Hertz<f32>,
    pub filter_q_value: f32,

    pub oscx_coarse: [f32 ;NB_OPERATORS],
    pub oscx_level: [f32; NB_OPERATORS],
    pub oscx_osc_type: [u8; NB_OPERATORS],
    pub oscx_phase_offset: [f32; NB_OPERATORS],
    pub oscx_feedback: [f32; NB_OPERATORS],
    pub oscx_adsr_attack: [f32; NB_OPERATORS],
    pub oscx_adsr_decay: [f32; NB_OPERATORS],
    pub oscx_adsr_release: [f32; NB_OPERATORS],
    pub oscx_adsr_sustain: [f32; NB_OPERATORS],
}