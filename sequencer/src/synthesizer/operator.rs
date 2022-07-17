use crate::adsr::ADSR;
use crate::noise::NOISE_TAB;
use crate::noise::NOISE_TAB_SIZE;

pub const SINE : u8 = 0;
pub const TRIANGLE : u8 = 1;
pub const SQUARE : u8 = 2;
pub const SAW_ANALOGIC_4 : u8 = 3;
pub const SAW_ANALOGIC_64 : u8 = 4;
pub const SAW_DIGITAL : u8 = 5;
pub const NOISE : u8 = 6;
pub const OSC_OFF : u8 = 7;

#[derive(Copy, Clone)]
pub struct Operator {
    frequency: f32,
    angular_speed: f32,
    current_angle: f32,
    pub volume: f32,
    pub adsr: ADSR,
    pub value: f32,
    pub osc_type: u8,
    feedback: f32,
    noise_index: usize,
    time: f32,
    time_step: f32,
}

impl Operator {
    pub fn new(sample_rate: f32, osc_type: u8) -> Operator {
        return Operator {
            frequency: 0.0,
            angular_speed: 0.0,
            current_angle: 0.0,
            volume: 1.0,
            adsr: ADSR::new(0.00243, 2.33, 1.0, 0.050, sample_rate),
            value: 0.0,
            osc_type: osc_type,
            noise_index: 0,
            feedback: 0.0,
            time: 0.,
            time_step: 0.,
        };
    }

    pub fn tick(&mut self) -> f32 {
        self.value = self.oscillate(self.current_angle) * self.adsr.tick() * self.volume;
        self.current_angle += self.angular_speed;

        if self.current_angle >= 2. * std::f32::consts::PI {
            self.current_angle -= 2. * std::f32::consts::PI;
        }

        if self.feedback == 0.0 {
            return self.value;
        } else {
            self.current_angle -= self.angular_speed;
            return self.tick_modulated(self.value * self.feedback);
        }
    }

    pub fn tick_modulated(&mut self, modulation: f32) -> f32 {
        self.value = self.oscillate(self.current_angle + modulation) * self.adsr.tick() * self.volume;
        self.current_angle += self.angular_speed;

        if self.current_angle > 2.0 * std::f32::consts::PI {
            self.current_angle -= 2.0 * std::f32::consts::PI;
        }

        return self.value;
    }

    pub fn init(&mut self, sample_rate: f32, frequency: f32, volume: f32, osc_type: u8, phase_offset: f32, feedback: f32) {
       self.frequency = frequency;
       self.angular_speed = (frequency / sample_rate) * 2.0 * std::f32::consts::PI;
       self.volume = volume;
       self.current_angle = self.angular_speed * (phase_offset * (1. / (frequency / sample_rate)));
       self.value = 0.0;
       self.noise_index = 0;
       self.osc_type = osc_type;
       self.feedback = feedback;
       self.time = 0.;
       self.time_step = 1. / sample_rate;
    }

    pub fn change_frequency_fly(&mut self, new_frequency: f32) {
        self.angular_speed = (new_frequency / 48000.) * 2.0 * std::f32::consts::PI;
        self.frequency = new_frequency;
    }

    pub fn oscillate(&mut self, phase: f32) -> f32 {

        if self.osc_type == SINE {
            return phase.sin();
        }

        if self.osc_type == TRIANGLE {
            return phase.sin().asin() * (2.0 / std::f32::consts::PI);
        }
        
        if self.osc_type == SQUARE {
            if phase.sin() > 0.0 {
                return 1.0;
            }
            return -1.0;
        }

        if self.osc_type == SAW_ANALOGIC_4 {
            let mut out = 0.;
            for n in 1..4
            {
                out += (n as f32 * phase).sin() / n as f32;
            }
            return out * 2.0 / std::f32::consts::PI;
        }

        if self.osc_type == SAW_ANALOGIC_64 {
            let mut out = 0.;
            for n in 1..64
            {
                out += (n as f32 * phase).sin() / n as f32;
            }
            return out * 2.0 / std::f32::consts::PI;
        }
        
        if self.osc_type == SAW_DIGITAL {
            let value = (2.0 / std::f32::consts::PI) * (self.frequency * std::f32::consts::PI * (self.time % (1.0 / self.frequency)) - (std::f32::consts::PI / 2.0));
            self.time += self.time_step;
            return value;
        }
        
        if self.osc_type == NOISE {
            let noise = NOISE_TAB[self.noise_index];
            self.noise_index += 1;
            if self.noise_index >= NOISE_TAB_SIZE {
                self.noise_index = 0;
            }
            return noise;
        }

        if self.osc_type == OSC_OFF {
            return 0.
        }
    
        return 0.;
    }
}

