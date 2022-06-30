pub struct Oscillator {
    sample_rate: f32,
    angle_delta: f32,
    frequency: f32,
    current_angle: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Oscillator {
        return Oscillator {
            sample_rate: sample_rate,
            angle_delta: 0.0,
            frequency: 0.0,
            current_angle: 0.0,
        };
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        let cycles_per_sample : f32 = frequency / self.sample_rate as f32;
        self.angle_delta = cycles_per_sample * 2.0 * std::f32::consts::PI;
    }

    pub fn get_sample(&mut self) -> f32 {
        let current_sample = (self.current_angle).sin();
        self.current_angle += self.angle_delta;
        return current_sample;
    }
}
