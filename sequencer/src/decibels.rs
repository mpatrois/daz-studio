
const MINUS_INFINITY_DB: f32 = -100.0;
pub const VOLUME_OFF: f32 = MINUS_INFINITY_DB;

pub fn db_to_gain (decibels: f32) -> f32
{
    if decibels > MINUS_INFINITY_DB {
        return f32::powf(10.0, decibels * 0.05);
    }
    return 0.0;
}