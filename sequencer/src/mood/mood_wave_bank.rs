use crate::utils::midi_note_fo_hertz;
use crate::mood::mood_wave::MoodWave;
use crate::mood::mood_wave::{
    WAVE_SINE,
    WAVE_TRIANGLE,
    WAVE_SAW_DIGITAL,
    WAVE_SAW_ANALOGIC_4,
    WAVE_SAW_ANALOGIC_256,
    WAVE_SQUARE_DIGITAL,
    WAVE_SQUARE_ANALOGIC_256,
    WAVE_NONE,
};

use std::rc::Rc;

pub struct MoodWaveBank {
    pub waves: Vec<Rc<MoodWave>>,
    pub footprint: usize
}

impl MoodWaveBank {
    pub fn new(sample_rate: f32) -> MoodWaveBank {

        let waves_forms_type = [
            WAVE_SINE,
            WAVE_TRIANGLE,
            WAVE_SAW_DIGITAL,
            WAVE_SAW_ANALOGIC_4,
            WAVE_SAW_ANALOGIC_256,
            WAVE_SQUARE_DIGITAL,
            WAVE_SQUARE_ANALOGIC_256,
        ];

        let mut footprint = 0;

        let mut waves : Vec<Rc<MoodWave>> = Vec::new();

        let twelve_root_of_two = f32::powf(2., 1./12.);

        for wave_form_type in waves_forms_type.iter() {
            // Each Wave Form Type Cover from A0 to A8
            let midi_a_minus_1 = 9;
            let midi_a_9 = 129;
            let mut root_frequency = midi_note_fo_hertz(midi_a_minus_1);
            let last_frequency = midi_note_fo_hertz(midi_a_9);

            while root_frequency <= last_frequency {
                let start_frequency = root_frequency * f32::powf(twelve_root_of_two, -6.);
                let end_frequency = root_frequency * f32::powf(twelve_root_of_two, 6.);
                let mood_wave = MoodWave::new(start_frequency, root_frequency, end_frequency, sample_rate, wave_form_type.clone());
                footprint += mood_wave.mood_wave_samples.len();
                root_frequency *= 2.;
                waves.push(Rc::new(mood_wave));
            }

            waves.push(Rc::new(MoodWave::new(1., 1., last_frequency, sample_rate, WAVE_NONE)))
        }        
        MoodWaveBank {
            waves,
            footprint
        }
    }

    pub fn find_mood_wave(&self, frequency: f32, mood_wave_form: u8) -> Result<Rc<MoodWave>, String> {
        for wave in self.waves.iter() {
            if wave.from_frequency <= frequency && frequency < wave.to_frequency && wave.wave_form == mood_wave_form {
                return Ok(wave.clone());
            }
        }
        println!("{}, {}, Not found", frequency, mood_wave_form);
        Err("Not found".to_string())
    }
}