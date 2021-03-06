use biquad::*;
use biquad::Type;
use crate::mood::mood_preset::MoodPreset;
use crate::fx::reverb::ReverbParameters;

use crate::mood::mood_wave::{
    WAVE_SINE,
    WAVE_SAW_DIGITAL,
    WAVE_SAW_ANALOGIC_4,
    WAVE_SAW_ANALOGIC_256,
    WAVE_SQUARE_ANALOGIC_256,
    WAVE_NONE,
};

pub fn get_mood_presets() -> Vec<MoodPreset> {
    return vec![
        MoodPreset { 
            id: 0,
            name: "G-Bass".to_string(),
            oscx_octave_range: [32., 8., 8.],
            oscx_semitone_shift: [0.0, 0.0, 0.0],
            oscx_volume: [0.8, 0., 0.],
            oscx_phase_offset: [0., 0., 0.],
            oscx_wave_form: [WAVE_SAW_ANALOGIC_256, WAVE_NONE, WAVE_NONE],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.1, 6.26, 0.145],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.05,
            filter_type: Type::LowPass,
            filter_f0: 5.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,
            
            reverb_enabled: false,
            reverb_params: ReverbParameters {
                room_size: 0.0,
                damping: 0.0,
                wet_level: 0.0,
                dry_level: 0.0,
                width: 0.0,
                freeze_mode: 0.0,
            }
        },
        MoodPreset { 
            id: 1,
            name: "G-Lead".to_string(),
            oscx_octave_range: [8., 8., 8.],
            oscx_semitone_shift: [0.0, -0.00, 0.0],
            oscx_volume: [0.52 , 0.53, 0.0],
            oscx_phase_offset: [0., 0., 0.],
            oscx_wave_form: [WAVE_SAW_ANALOGIC_256, WAVE_SAW_ANALOGIC_256, WAVE_NONE],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.1, 0.1, 0.1],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.07,
            filter_type: Type::LowPass,
            filter_f0: 5.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,

            reverb_enabled: true,
            reverb_params: ReverbParameters {
                room_size: 0.2,
                damping: 0.5,
                wet_level: 0.2,
                dry_level: 0.4,
                width: 0.4,
                freeze_mode: 0.0,
            }

        },
        MoodPreset { 
            id: 2,
            name: "Thrill Bass".to_string(),
            oscx_octave_range: [32., 32., 16.],
            oscx_semitone_shift: [0.0, 0., -0.1],
            oscx_volume: [0.4, 0.4, 0.4],
            oscx_phase_offset: [0., 0., 0.],
            oscx_wave_form: [WAVE_SAW_DIGITAL, WAVE_SQUARE_ANALOGIC_256, WAVE_SAW_DIGITAL],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.1, 0.1, 0.1],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.005,
            filter_type: Type::LowPass,
            filter_f0: 1.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,

            reverb_enabled: false,
            reverb_params: ReverbParameters {
                room_size: 0.0,
                damping: 0.0,
                wet_level: 0.0,
                dry_level: 0.0,
                width: 0.0,
                freeze_mode: 0.0,
            }
        },
        MoodPreset {
            id: 3,
            name: "Guitar Bass".to_string(),
            oscx_octave_range: [32., 16., 32.],
            oscx_semitone_shift: [0.0, 0., 0.0],
            oscx_volume: [0.2, 0.45, 0.7],
            oscx_phase_offset: [0., 0.6, 0.],
            oscx_wave_form: [WAVE_SAW_ANALOGIC_4, WAVE_SINE, WAVE_SAW_ANALOGIC_4],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.08, 0.08, 0.08],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.005,
            filter_type: Type::LowPass,
            filter_f0: 800.hz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,

            reverb_enabled: false,
            reverb_params: ReverbParameters {
                room_size: 0.0,
                damping: 0.0,
                wet_level: 0.0,
                dry_level: 0.0,
                width: 0.0,
                freeze_mode: 0.0,
            }
        },
        MoodPreset {
            id: 4,
            name: "Sine".to_string(),
            oscx_octave_range: [2., 16., 32.],
            oscx_semitone_shift: [0.0, 0., 0.0],
            oscx_volume: [0.5, 0.0, 0.0],
            oscx_phase_offset: [0., 0.6, 0.],
            oscx_wave_form: [WAVE_SINE, WAVE_NONE, WAVE_NONE],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.08, 0.08, 0.08],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.08,
            filter_type: Type::LowPass,
            filter_f0: 10.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,
            
            reverb_enabled: true,
            reverb_params: ReverbParameters {
                room_size: 0.2,
                damping: 0.5,
                wet_level: 0.2,
                dry_level: 0.4,
                width: 1.0,
                freeze_mode: 0.0,
            }
        },
        MoodPreset {
            id: 4,
            name: "Sine 2".to_string(),
            oscx_octave_range: [8., 16., 32.],
            oscx_semitone_shift: [0.0, 0., 0.0],
            oscx_volume: [0.5, 0.0, 0.0],
            oscx_phase_offset: [0., 0.6, 0.],
            oscx_wave_form: [WAVE_SINE, WAVE_NONE, WAVE_NONE],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.08, 0.08, 0.08],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.08,
            filter_type: Type::LowPass,
            filter_f0: 10.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: true,

            reverb_enabled: true,
            reverb_params: ReverbParameters {
                room_size: 0.2,
                damping: 0.5,
                wet_level: 0.33,
                dry_level: 0.4,
                width: 1.0,
                freeze_mode: 0.0,
            }
        },
        MoodPreset {
            id: 5,
            name: "Poly".to_string(),
            oscx_octave_range: [4., 2., 1.],
            oscx_semitone_shift: [0.0, 0., 0.0],
            oscx_volume: [0.5, 0.2, 0.4],
            oscx_phase_offset: [0., 0.6, 0.],
            oscx_wave_form: [WAVE_SINE, WAVE_SINE, WAVE_SINE],
            oscx_adsr_attack: [0.00423, 0.00423, 0.00423],
            oscx_adsr_decay: [0.38, 0.969, 60.],
            oscx_adsr_release: [0.08, 0.08, 0.08],
            oscx_adsr_sustain: [1.0, 1.0, 1.0],
            glide: 0.0,
            filter_type: Type::LowPass,
            filter_f0: 10.khz(),
            filter_q_value: biquad::Q_BUTTERWORTH_F32,
            is_mono: false,

            reverb_enabled: false,
            reverb_params: ReverbParameters {
                room_size: 0.2,
                damping: 0.5,
                wet_level: 0.33,
                dry_level: 0.4,
                width: 1.0,
                freeze_mode: 0.0,
            }
        },
    ];
}