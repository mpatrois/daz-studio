
const NB_OPERATORS : usize = 4;

use crate::preset::Preset;

#[derive(Clone)]
pub struct SamplerPreset {
    pub id: usize,
    pub name: String,
}


impl Preset for SamplerPreset {
    fn get_id(self) -> usize {
        return self.id;
    }

    fn get_name(self) -> String {
        return self.name;
    }
}