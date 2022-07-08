pub trait Preset {
    fn get_id(self) -> usize;
    fn get_name(&self) -> String;
}