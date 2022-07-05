#[derive(PartialEq, Eq, Copy, Clone)]
enum ADSRState { 
    IDLE = 1, 
    ATTACK = 2, 
    DECAY = 3, 
    SUSTAIN = 4, 
    RELEASE = 5 
}

#[derive(Copy, Clone)]
pub struct ADSR {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    sample_rate: f32,
    pub envelope_val: f32,
    attack_rate: f32,
    decay_rate: f32,
    release_rate: f32,
    state: ADSRState
}

impl ADSR {
    pub fn new(attack: f32, decay: f32, sustain: f32, release:f32, sample_rate: f32) -> ADSR {
        let mut adsr = ADSR {
            attack: attack,
            decay: decay,
            sustain: sustain,
            release: release,
            envelope_val: 0.0,
            attack_rate: 0.0,
            decay_rate: 0.0,
            release_rate: 0.0,
            sample_rate: sample_rate,
            state: ADSRState::IDLE
        };
        adsr.recalculate_rates();
        // {todo} The `return` keyword is useless here.
        return adsr;
    }

    pub fn set_adsr(&mut self, attack: f32, decay: f32, sustain: f32, release:f32) {
        self.attack = attack;
        self.decay = decay;
        self.sustain = sustain;
        self.release = release;
        self.recalculate_rates();
        self.reset();
    }

    pub fn get_rate(&self, distance: f32, time_in_seconds: f32, sr: f32) -> f32 {
        if time_in_seconds > 0.0 {
            // {todo} The `return` keyword is useless here.
            return distance / (time_in_seconds * sr);
        } else {
            // {todo} The `return` keyword is useless here.
            return -1.0;
        }
    }

    pub fn recalculate_rates(&mut self) {
        self.attack_rate = self.get_rate(1.0, self.attack, self.sample_rate);
        self.decay_rate = self.get_rate(1.0 - self.sustain, self.decay, self.sample_rate);
        self.release_rate = self.get_rate(self.sustain, self.release, self.sample_rate);

        if (self.state == ADSRState::ATTACK && self.attack_rate <= 0.0)
            || (self.state == ADSRState::DECAY && (self.decay_rate <= 0.0 || self.envelope_val <= self.sustain))
            || (self.state == ADSRState::RELEASE && self.release_rate <= 0.0)
        {
            self.go_to_next_state();
        }
    }

    pub fn reset(&mut self) {
        self.envelope_val = 0.0;
        self.state = ADSRState::IDLE;
    }

    pub fn note_on(&mut self) {
        if self.attack_rate > 0.0 {
            self.state = ADSRState::ATTACK;
        } else if self.decay_rate > 0.0 {
            self.envelope_val = 1.0;
            self.state = ADSRState::DECAY;
        } else {
            self.envelope_val = self.sustain;
            self.state = ADSRState::SUSTAIN;
        }
    }

    pub fn note_off(&mut self) {
        if self.state != ADSRState::IDLE {
            if self.release > 0.0 {
                self.release_rate = self.envelope_val / (self.release * self.sample_rate);
                self.state = ADSRState::RELEASE;
            } else {
                self.reset();
            }
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.recalculate_rates();
    }

    // {todo} It's generally better to be stateless (and to avoid
    // `mut`, `&mut` or `&mut self` where reasonably possible). `ADSR`
    // is simple yet state-full. It seems like `ADSR` is used not only
    // as an envelop generator but also as a tracker for active notes.
    //
    // A better solution might be to track active notes separately
    // (probably in `sequencer.rs`). Tracking an active note could be
    // done easily by storing the `Note On` time along with other
    // informations such as pitch or velocity in a dedicated `struct`
    // like `Note`. Tracking active notes separately is also useful
    // because the envelop might not be the only thing that is
    // changing depending on the time elapsed since the `Note On`.
    //
    // This new `Note` `struct` could even be reused to store not only
    // current active notes but also notes in a sequence by adding an
    // optional `Note Off` time... One does not necessarily need to
    // store sequences in MIDI nor in a format that maps one-to-one
    // with MIDI...
    //
    // Anyway, `ADSR` could be mostly read-only and stateless, with
    // maybe an exception for the sample rate if cached computations
    // are needed. `ADSR` could simply provide a method taking the
    // time elapsed since the `Note On` and returning an envelop gain,
    // for exemple:
    //
    // `pub fn envelop(&self, t: f32) -> f32 {...}`
    //
    // The time elapsed since the `Note On` is trivially computed from
    // the current time and the `Note On` time stored as described
    // above.
    pub fn tick(&mut self) -> f32 {
        if self.state == ADSRState::IDLE {
            return 0.0;
        }

        if self.state == ADSRState::ATTACK {
            self.envelope_val += self.attack_rate;
            if self.envelope_val >= 1.0 {
                self.envelope_val = 1.0;
                self.go_to_next_state();
            }
        } else if self.state == ADSRState::DECAY {
            self.envelope_val -= self.decay_rate;
            if self.envelope_val <= self.sustain {
                self.envelope_val = self.sustain;
                self.go_to_next_state();
            }
        } else if self.state == ADSRState::SUSTAIN {
            self.envelope_val = self.sustain;
        }
        else if self.state == ADSRState::RELEASE {
            self.envelope_val -= self.release_rate;
            if self.envelope_val <= 0.0 {
                self.go_to_next_state();
            }
        }
        // {todo} The `return` keyword is useless here.
        return self.envelope_val;
    }

    pub fn go_to_next_state(&mut self) {
        if self.state == ADSRState::ATTACK {
            if self.decay_rate > 0.0 {
                self.state = ADSRState::DECAY;
            } else {
                self.state = ADSRState::SUSTAIN;
            }
        }
        else if self.state == ADSRState::DECAY {
            self.state = ADSRState::SUSTAIN;
        }
        else if self.state == ADSRState::RELEASE {
            self.reset();
        }
    }
    // pub fn is_idle(&self) -> bool {
    //     return self.state == ADSRState::IDLE;
    // }
}