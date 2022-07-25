use crate::preset::Preset;


#[derive(Clone)]
pub struct EpianoPreset {
    pub id: usize,
    pub name: String,
    pub envelope_decay: f32,
    pub envelope_release: f32,
    pub hardness: f32,
    pub treble_boost: f32,
    pub modulation: f32,
    pub lfo_rate: f32,
    pub velocity_sense: f32,
    pub stereo_width: f32,
    pub polyphony: f32,
    pub fine_tuning: f32,
    pub random_tuning: f32,
    pub overdrive: f32,
}

impl EpianoPreset {
    pub fn empty() -> EpianoPreset {
        EpianoPreset {
            id: 0,
            name: "Default".to_string(),
            envelope_decay: 0.500,
            envelope_release: 0.500,
            hardness: 0.500,
            treble_boost: 0.500,
            modulation: 0.500,
            lfo_rate: 0.650,
            velocity_sense: 0.250,
            stereo_width: 0.500,
            polyphony: 0.50,
            fine_tuning: 0.500,
            random_tuning: 0.146,
            overdrive: 0.000,
        }
    }
}

impl Preset for EpianoPreset {
    fn get_id(self) -> usize {
        return self.id;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
