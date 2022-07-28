// https://ccrma.stanford.edu/~jos/pasp/Freeverb.html

const NB_COMBS_FILTERS : usize = 8; 
const NB_ALL_PASS_FILTERS : usize = 4;
const NB_CHANNELS : usize= 2;

#[derive(Clone)]
pub struct ReverbParameters {
    pub room_size: f32,
    pub damping: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub width: f32,
    pub freeze_mode: f32,
}

impl ReverbParameters {
    pub fn new() -> ReverbParameters {
        ReverbParameters {
            room_size: 0.5,
            damping: 0.5,
            wet_level: 0.33,
            dry_level: 0.4,
            width: 1.0,
            freeze_mode: 0.0,
        }
    }
}

#[derive(Clone)]
struct CombFilter {
    buffer: Vec<f32>,
    buffer_size: usize,
    buffer_index: usize,
    last: f32
}

impl CombFilter {
    pub fn new() -> CombFilter {
        CombFilter {
            buffer: Vec::new(),
            buffer_index: 0,
            buffer_size: 0,
            last: 0.,
        }
    }

    pub fn set_size(&mut self, size: usize) {
        if size != self.buffer_size {
            self.buffer_index = 0;
            self.buffer = Vec::with_capacity(size);
            self.buffer_size = size;
            for _i in 0..self.buffer_size {
                self.buffer.push(0.0);
            } 
        }
        self.clear();
    }

    pub fn clear(&mut self) {
        self.last = 0.;
        for s in &mut self.buffer {
            *s = 0.;
        }
    }

    pub fn process(&mut self, input: f32, damp: f32, feedback_level: f32) -> f32
    {
        let output = self.buffer[self.buffer_index];
        self.last = (output * (1.0 - damp)) + (self.last * damp);
        self.last += 0.1; self.last -= 0.1;

        let mut temp = input + (self.last * feedback_level);
        temp += 0.1; temp -= 0.1;

        self.buffer[self.buffer_index] = temp;
        self.buffer_index = (self.buffer_index + 1) % self.buffer_size;
        return output;
    }
}

#[derive(Clone)]
struct AllPassFilter {
    buffer: Vec<f32>,
    buffer_size: usize,
    buffer_index: usize
}

impl AllPassFilter {

    pub fn new() -> AllPassFilter {
        AllPassFilter {
            buffer: Vec::new(),
            buffer_index: 0,
            buffer_size: 0,
        }
    }

    pub fn set_size(&mut self, size: usize)
    {
        if size != self.buffer_size {
            self.buffer_index = 0;
            self.buffer = Vec::with_capacity(size);
            self.buffer_size = size;
            for _i in 0..self.buffer_size {
                self.buffer.push(0.0);
            }
        }
        self.clear();
    }

    pub fn clear(&mut self) {
        for s in &mut self.buffer {
            *s = 0.;
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let buffered_value = self.buffer[self.buffer_index];
        let mut temp = input + (buffered_value * 0.5);
        temp += 0.1; temp -= 0.1;

        self.buffer [self.buffer_index] = temp;
        self.buffer_index = (self.buffer_index + 1) % self.buffer_size;
        return buffered_value - input;
    }
}

#[derive(Clone)]
struct SmoothedValue {
    current_value: f32,
    target: f32,
    steps_to_target: i32,
    step: f32,
    countdown: i32
}

impl SmoothedValue {

    pub fn new() -> SmoothedValue {
        SmoothedValue {
            current_value: 0.,
            target: 0.,
            countdown: 0,
            steps_to_target: 0,
            step: 0.
        }
    }

    pub fn is_smoothing(&self) -> bool {
        return self.countdown > 0;
    }

    pub fn set_next_value(&mut self) {
        self.current_value += self.step;
    } 

    pub fn get_next_value(&mut self) -> f32 {
        if !self.is_smoothing() {
            return self.target;
        }

        self.countdown -= 1;

        if self.is_smoothing() {
            self.set_next_value();
        }
        else {
            self.current_value = self.target;
        }

        return self.current_value;
    }

    pub fn set_target_value(&mut self, new_value: f32) {

        if new_value == self.target {
            return;
        }

        if self.steps_to_target <= 0 {
            self.set_current_and_target_value(new_value);
            return;
        }

        self.target = new_value;
        self.countdown = self.steps_to_target;

        self.set_step_size();
    }

    pub fn reset(&mut self, sample_rate: f32, ramp_length_in_seconds: f64) {
        self.steps_to_target = (ramp_length_in_seconds * sample_rate as f64).floor() as i32;
        self.set_current_and_target_value(self.target);
    }

    pub fn set_current_and_target_value(&mut self, new_value: f32) {
        self.current_value = new_value;
        self.target = self.current_value;
        self.countdown = 0;
    }

    pub fn set_step_size(&mut self) {
        self.step = (self.target - self.current_value) / self.countdown as f32;
    }
}

#[derive(Clone)]
pub struct Reverb {
    parameters: ReverbParameters,
    gain: f32,
    comb: [[CombFilter; NB_COMBS_FILTERS]; NB_CHANNELS],
    all_pass: [[AllPassFilter; NB_ALL_PASS_FILTERS]; NB_CHANNELS],

    damping: SmoothedValue,
    feedback: SmoothedValue,
    dry_gain: SmoothedValue,
    wet_gain1: SmoothedValue,
    wet_gain2: SmoothedValue,
}

impl Reverb {
    pub fn new(sample_rate: f32) -> Reverb {
        
        let parameters = ReverbParameters::new();
        let mut reverb = Reverb {
            gain: 0.,
            parameters,
            comb: [
                [
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(),
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(),
                ],
                [
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(),
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(), 
                    CombFilter::new(),
                ],
            ],
            all_pass: [
                [
                    AllPassFilter::new(),
                    AllPassFilter::new(),
                    AllPassFilter::new(),
                    AllPassFilter::new(),
                ],
                [
                    AllPassFilter::new(),
                    AllPassFilter::new(),
                    AllPassFilter::new(),
                    AllPassFilter::new(),
                ]
            ],
            damping: SmoothedValue::new(),
            feedback: SmoothedValue::new(),
            dry_gain: SmoothedValue::new(),
            wet_gain1: SmoothedValue::new(),
            wet_gain2: SmoothedValue::new()
        };
        reverb.set_parameters(ReverbParameters::new());
        reverb.set_sample_rate(sample_rate);
        
        reverb
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32)
    {
        let comb_tunings : [i32; 8] = [ 1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617 ];
        let all_pass_tunings: [i32; 4] = [ 556, 441, 341, 225 ];
        let stereo_spread = 23;
        let int_sample_rate = sample_rate as i32;

        for i in 0..NB_COMBS_FILTERS {
            self.comb[0][i].set_size((int_sample_rate * comb_tunings[i]) as usize / 44100);
            self.comb[1][i].set_size((int_sample_rate * (comb_tunings[i] + stereo_spread)) as usize / 44100);
        }

        for i in 0..NB_ALL_PASS_FILTERS {
            self.all_pass[0][i].set_size((int_sample_rate * all_pass_tunings[i]) as usize / 44100);
            self.all_pass[1][i].set_size((int_sample_rate * (all_pass_tunings[i] + stereo_spread)) as usize / 44100);
        }

        let smooth_time: f64 = 0.01;
        self.damping.reset(sample_rate, smooth_time);
        self.feedback.reset(sample_rate, smooth_time);
        self.dry_gain.reset(sample_rate, smooth_time);
        self.wet_gain1.reset(sample_rate, smooth_time);
        self.wet_gain2.reset(sample_rate, smooth_time);
    }

    pub fn set_parameters(&mut self, new_params: ReverbParameters) {
        let wet_scale_factor = 3.0;
        let dry_scale_factor = 2.0;

        let wet = new_params.wet_level * wet_scale_factor;

        self.dry_gain.set_target_value(new_params.dry_level * dry_scale_factor);
        self.wet_gain1.set_target_value(0.5 * wet * (1.0 + new_params.width));
        self.wet_gain2.set_target_value (0.5 * wet * (1.0 - new_params.width));

        if self.is_frozen(new_params.freeze_mode) {
            self.gain = 0.0;
        } else {
            self.gain = 0.015;
        } 
        self.parameters = new_params;
        self.update_damping();
    }

    pub fn is_frozen(&self, freeze_mode: f32) -> bool { 
        return freeze_mode >= 0.5; 
    }

    pub fn update_damping(&mut self) {
        let room_scale_factor = 0.28;
        let room_offset = 0.7;
        let damp_scale_factor = 0.4;

        if self.is_frozen(self.parameters.freeze_mode) {
            self.set_damping(0.0, 1.0);
        }
        else {
            self.set_damping(
                self.parameters.damping * damp_scale_factor,
                self.parameters.room_size * room_scale_factor + room_offset
            );
        }
    }

    pub fn set_damping(&mut self, damping_to_use: f32, room_size_to_use: f32) {
        self.damping.set_target_value(damping_to_use);
        self.feedback.set_target_value(room_size_to_use);
    }

    pub fn process(&mut self, outputs: &mut [f32], num_samples: usize) {
        let mut idx = 0;

        for _i in 0..num_samples {
            let input = (outputs[idx] + outputs[idx+1]) * self.gain;
            let mut out_l = 0.;
            let mut out_r = 0.;

            let damp = self.damping.get_next_value();
            let feedbck = self.feedback.get_next_value();

            for j in 0..NB_COMBS_FILTERS {
                out_l += self.comb[0][j].process(input, damp, feedbck);
                out_r += self.comb[1][j].process(input, damp, feedbck);
            }
        
            for j in 0..NB_ALL_PASS_FILTERS {
                out_l = self.all_pass[0][j].process(out_l);
                out_r = self.all_pass[1][j].process(out_r);
            }

            let dry = self.dry_gain.get_next_value();
            let wet1 = self.wet_gain1.get_next_value();
            let wet2 = self.wet_gain2.get_next_value();

            outputs[idx]  = out_l * wet1 + out_r * wet2 + outputs[idx] * dry;
            outputs[idx+1] = out_r * wet1 + out_l * wet2 + outputs[idx+1] * dry;
            idx += 2;
        }
    }

    pub fn reset(&mut self) {
        for j in  0..NB_CHANNELS {
            for i in 0..NB_COMBS_FILTERS {
                self.comb[j][i].clear();
            }
            for i in 0..NB_ALL_PASS_FILTERS {
                self.all_pass[j][i].clear();
            }
        }
    }
}