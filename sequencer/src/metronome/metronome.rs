use crate::oscillator::Oscillator;
use crate::metronome::metronome_voice::MetronomeVoice;

const MAX_NOTES: usize = 8;

pub struct Metronome {
    buffer_bip_1: Vec<f32>,
    buffer_bip_2: Vec<f32>,
    voices: Vec<MetronomeVoice>,
    nb_actives_notes: usize
}

impl Metronome {
    pub fn new(sample_rate: f32) -> Metronome {

        let mut buffer_bip_1: Vec<f32> = Vec::new();
        let mut buffer_bip_2: Vec<f32> = Vec::new();
        let mut oscillator1 = Oscillator::new(sample_rate);
        let mut oscillator2 = Oscillator::new(sample_rate);

        oscillator1.set_frequency(1000.0);
        oscillator2.set_frequency(700.0);
        // let mut adsr = ADSR::new(0.01, 0.0, 1.0, 0.05, sample_rate);
        let mut voices : Vec<MetronomeVoice> = Vec::new();

        for _i in 0..MAX_NOTES {
            voices.push(MetronomeVoice::new())
        }

        for _i in 0..(40.0 / 1000.0 * sample_rate) as i32 {
            buffer_bip_1.push(oscillator1.get_sample() * 0.3);
            buffer_bip_2.push(oscillator2.get_sample() * 0.3);
        }

        return Metronome {
            buffer_bip_1: buffer_bip_1,
            buffer_bip_2: buffer_bip_2,
            voices: voices,
            nb_actives_notes: 0
        };
    }

    pub fn bip(&mut self, start_bar: bool) {
        if self.nb_actives_notes < MAX_NOTES - 1 {
            let note_to_active = self.nb_actives_notes as usize; 
            self.voices[note_to_active].active = true;
            if start_bar {
                self.voices[note_to_active].buffer = &self.buffer_bip_1;
            } else {
                self.voices[note_to_active].buffer = &self.buffer_bip_2;
            }
            self.voices[note_to_active].buffer_index = 0;
            self.nb_actives_notes += 1;
        }
    }

    pub fn process(&mut self, outputs: &mut [f32], num_samples: usize, nb_channels: usize) {
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if !self.voices[i].is_ended() {
                self.voices[i].render_next_block(outputs, num_samples, nb_channels);
            }
        }
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if self.voices[i].is_ended() {
                self.nb_actives_notes -= 1;
                let active_notes = self.nb_actives_notes as usize;
                self.voices[i].buffer_index = self.voices[active_notes].buffer_index;
                self.voices[i].buffer = self.voices[active_notes].buffer;
                self.voices[i].active = self.voices[active_notes].active;
            } 
        }
    }
}