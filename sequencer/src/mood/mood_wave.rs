pub const WAVE_SINE: u8 = 0;
pub const WAVE_TRIANGLE: u8 = 1;
pub const WAVE_SAW_DIGITAL: u8 = 2;
pub const WAVE_SAW_ANALOGIC_4: u8 = 3;
pub const WAVE_SAW_ANALOGIC_256: u8 = 4;
pub const WAVE_SQUARE_DIGITAL: u8 = 5;
pub const WAVE_SQUARE_ANALOGIC_256: u8 = 6;
pub const WAVE_NONE: u8 = 7;

#[derive(Clone)]
pub struct MoodWave {
    pub mood_wave_samples: Vec<f32>,
    pub from_frequency: f32,
    pub to_frequency: f32,
    pub root_frequency: f32,
    pub wave_form: u8
}

impl MoodWave {
    pub fn empty() -> MoodWave {
        MoodWave {
            from_frequency: 0.,
            to_frequency: 0.,
            root_frequency: 0.,
            mood_wave_samples: Vec::new(),
            wave_form: WAVE_NONE
        }
    }

    pub fn new(from_frequency: f32, root_frequency: f32, to_frequency: f32, sample_rate: f32, wave_form: u8) -> MoodWave {

        let nb_samples_period = 1. / ((root_frequency / sample_rate)) as f32;
        let mut mood_wave_samples = Vec::with_capacity(nb_samples_period as usize);
        
        let mut time : f32 = 0.0;
        let mut phase : f32 = 0.0;

        let angular_speed = (root_frequency / sample_rate) * 2.0 * std::f32::consts::PI;
        let time_step = 1. / sample_rate;

        for _ in 0..nb_samples_period as usize {
            let mut out = 0.0;
            match wave_form {
                WAVE_SINE => {
                   out = phase.sin();
                },
                WAVE_TRIANGLE => {
                   out = phase.sin().asin() * (2.0 / std::f32::consts::PI);
                },
                WAVE_SAW_DIGITAL => {
                   out = (2.0 / std::f32::consts::PI) * (root_frequency * std::f32::consts::PI * (time % (1.0 / root_frequency)) - (std::f32::consts::PI / 2.0));
                },
                WAVE_SAW_ANALOGIC_4 => {
                    out = make_saw_analogic(4, phase, true)
                },
                WAVE_SAW_ANALOGIC_256 => {
                    out = make_saw_analogic(256, phase, true)
                },
                WAVE_SQUARE_DIGITAL => {
                    if phase.sin() > 0.0 {
                        out = 1.0;
                    } else {
                        out = -1.0;
                    }
                },
                WAVE_SQUARE_ANALOGIC_256 => {
                    let mut n = 1.0;
                    for _ in 0..256 {
                        out += (1. / n) * (n * phase).sin();
                        n += 2.;
                    }
                    out = 4. / std::f32::consts::PI * out;
                },
                _ => {
                    out = 0.;
                },
            };

            phase += angular_speed;
            time += time_step;

            mood_wave_samples.push(out * 0.5);
        }

        MoodWave {
            from_frequency,
            to_frequency,
            root_frequency,
            mood_wave_samples,
            wave_form: wave_form
        }
    }
}

pub fn make_saw_analogic(nb_series: usize, phase: f32, reverse: bool) -> f32 {
    let mut out = 0.0;
    let mut n = 1.0;
    for _ in 0..nb_series {
        out += f32::powf(-1.0, n) * (n as f32 * phase).sin() / n as f32;
        n += 1.0;
    }

    if reverse {
        std::f32::consts::FRAC_2_PI * out
    } else {
        (1. / 2.) - (std::f32::consts::FRAC_1_PI * out)
    }
}