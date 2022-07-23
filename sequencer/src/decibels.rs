
const MINUS_INFINITY_DB: f32 = -100.0;
pub const VOLUME_OFF: f32 = MINUS_INFINITY_DB;

pub fn db_to_gain (decibels: f32) -> f32
{
    if decibels > MINUS_INFINITY_DB {
        return f32::powf(10.0, decibels * 0.05);
    }
    return 0.0;
}


pub fn root_mean_square_stereo(outputs: &[f32], num_samples: usize) -> [f32; 2] {
    let mut rms_left = 0.;
    let mut rms_right = 0.;

    for i in 0..(outputs.len() / 2) {
        rms_left += outputs[i] * outputs[i];
        rms_right += outputs[i + 1] * outputs[i + 1];
    }

    [
        db_to_gain((rms_left / num_samples as f32).sqrt().log10() * 10.), 
        db_to_gain((rms_right / num_samples as f32).sqrt().log10() * 10.),
    ]
}