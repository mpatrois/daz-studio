use crate::preset::Preset;

use biquad::*;
use biquad::Type;
use crate::fx::reverb::ReverbParameters;

const NB_OSCILLATORS : usize = 3;

use crate::mood::mood_wave::{
    WAVE_NONE,
};

#[derive(Clone)]
pub struct MoodPreset {
    pub id: usize,
    pub name: String,
    pub oscx_octave_range: [f32; NB_OSCILLATORS],
    pub oscx_semitone_shift: [f32; NB_OSCILLATORS],
    pub oscx_wave_form: [u8; NB_OSCILLATORS],
    pub oscx_volume: [f32; NB_OSCILLATORS],
    pub oscx_phase_offset: [f32; NB_OSCILLATORS],
    pub oscx_adsr_attack: [f32; NB_OSCILLATORS],
    pub oscx_adsr_decay: [f32; NB_OSCILLATORS],
    pub oscx_adsr_release: [f32; NB_OSCILLATORS],
    pub oscx_adsr_sustain: [f32; NB_OSCILLATORS],
    pub glide: f32,
    pub filter_type: biquad::Type<f32>,
    pub filter_f0: biquad::Hertz<f32>,
    pub filter_q_value: f32,
    pub is_mono: bool,
    pub modulation_3_to_2_and_1: bool,

    pub reverb_enabled: bool,
    pub reverb_params: ReverbParameters,
}

impl MoodPreset {
    pub fn empty() -> MoodPreset {
        MoodPreset {
            id: 0,
            name: "Empty".to_string(),
            oscx_octave_range: [4., 2., 8.],
            oscx_semitone_shift: [0.0, -0.09, 0.0],
            oscx_wave_form: [WAVE_NONE, WAVE_NONE, WAVE_NONE],
            oscx_volume: [0.82 , 0.83, 0.0],
            oscx_phase_offset: [0., 0., 0.],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.1, 0.1, 0.1],
            oscx_adsr_sustain: [0., 0., 0.],
            
            glide: 0.,
            filter_type: Type::LowPass,
            filter_f0: 10.khz() as biquad::Hertz<f32>,
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,
            modulation_3_to_2_and_1: false,
            
            reverb_enabled: false,
            reverb_params: ReverbParameters {
                room_size: 0.5,
                damping: 0.5,
                wet_level: 0.33,
                dry_level: 0.4,
                width: 1.0,
                freeze_mode: 0.0,
            }
        }
    }
}

impl Preset for MoodPreset {
    fn get_id(self) -> usize {
        return self.id;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
