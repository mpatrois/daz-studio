use crate::synthesizer::operator::Operator;
use crate::synthesizer::operator::SINE;

use biquad::*;
use biquad::Type;
use biquad::DirectForm1;

const NB_OPERATORS : usize = 4;

pub const OP_A : usize = 3;
pub const OP_B : usize = 2;
pub const OP_C : usize = 1;
pub const OP_D : usize = 0;

#[derive(Copy, Clone)]
pub struct SynthesizerVoice {
    pub active: bool,
    pub note_id: u8,
    pub operators: [Operator; NB_OPERATORS],
    pub sample_rate: f32,
    pub algorithm: u8,
    pub biquad_filter: DirectForm1::<f32>,
    pub filter_type: biquad::Type<f32>,
    pub filter_f0: biquad::Hertz<f32>,
    pub filter_q_value: f32
}

impl SynthesizerVoice {
    pub fn new(sample_rate: f32) -> SynthesizerVoice {
        
        let f0 = 880.hz();
        let fs = 30.khz();

        let coeffs = Coefficients::<f32>::from_params(Type::AllPass, fs, f0, biquad::Q_BUTTERWORTH_F32).unwrap();
        let biquad_filter = DirectForm1::<f32>::new(coeffs);

        return SynthesizerVoice {
            active: true,
            note_id: 0,
            operators: [
                Operator::new(sample_rate, SINE),
                Operator::new(sample_rate, SINE),
                Operator::new(sample_rate, SINE),
                Operator::new(sample_rate, SINE),
            ],
            sample_rate: sample_rate,
            algorithm: 5,
            biquad_filter: biquad_filter,
            filter_type: Type::AllPass,
            filter_f0: f0,
            filter_q_value: biquad::Q_BUTTERWORTH_F32
        };
    }

    pub fn start_note(&mut self, midi_note :u8, _velocity: f32) {
        let coeffs = Coefficients::<f32>::from_params(self.filter_type, self.sample_rate.hz(), self.filter_f0, biquad::Q_BUTTERWORTH_F32).unwrap();
        self.biquad_filter = DirectForm1::<f32>::new(coeffs);
        
        self.biquad_filter.reset_state();
        self.note_id = midi_note;
        for i in 0..NB_OPERATORS {
            self.operators[i].adsr.reset();
            self.operators[i].adsr.note_on();
        }
        self.active = true;
    }

    pub fn stop_note(&mut self) {
        for i in 0..NB_OPERATORS {
            self.operators[i].adsr.note_off();
        }
    }

    pub fn render_next_block(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize) { 
        let mut out = 0.0;
        let mut idx = 0;
        while idx < nb_channels * num_samples {

            if self.algorithm == 1 {
                self.operators[OP_D].tick();
                self.operators[OP_C].tick_modulated(self.operators[OP_D].value);
                self.operators[OP_B].tick_modulated(self.operators[OP_C].value);
                let op0 = self.operators[OP_A].tick_modulated(self.operators[OP_B].value);
                out = op0;
            }

            if self.algorithm == 5 {
                self.operators[OP_D].tick();
                self.operators[OP_C].tick_modulated(self.operators[OP_D].value);
                let op1 = self.operators[OP_B].tick_modulated(self.operators[OP_C].value);
                let op0 = self.operators[OP_A].tick_modulated(self.operators[OP_C].value);
                out = op1 + op0;
            }

            if self.algorithm == 6 {
                self.operators[OP_D].tick();
                self.operators[OP_C].tick_modulated(self.operators[OP_D].value);
                let op1 = self.operators[OP_B].tick_modulated(self.operators[OP_C].value);
                let op0 = self.operators[OP_A].tick();
                out = op0 + op1;
            }

            if self.algorithm == 8 {
                self.operators[OP_D].tick();
                let op_c = self.operators[OP_C].tick_modulated(self.operators[OP_D].value);
                self.operators[OP_B].tick();
                let op_a = self.operators[OP_A].tick_modulated(self.operators[OP_B].value);
                out = op_a + op_c;
            }

            if self.algorithm == 11 {
                let op3 = self.operators[OP_D].tick();
                let op2 = self.operators[OP_C].tick();
                let op1 = self.operators[OP_B].tick();
                let op0 = self.operators[OP_A].tick();
                out = op3 + op2 + op1 + op0;
            }

            out = self.biquad_filter.run(out);

            outputs[idx] += out;
            outputs[idx + 1] += out;

            idx += 2;
        }
        
        if out.abs() <= 0.000000000001 {
            
            self.active = false;
        }
    }

    pub fn is_ended(&self) -> bool {
        return !self.active;
    }
}