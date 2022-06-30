use crate::processor::Processor;
use crate::midimessage::MidiMessage;
use crate::sampler::sample::Sample;
use crate::sampler::sample_voice::SamplerVoice;

const MAX_NOTES : usize = 32;

pub struct Sampler {
    // {todo} Try to avoid using `pub` everywhere and use methods for
    // encapsulation.
    pub samples: Vec<Sample>,
    voices: Vec<SamplerVoice>,
    nb_actives_notes: usize,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub id: i32,
    pub note_events: Vec<MidiMessage>,
    pub im_armed: bool
}

impl Sampler {
    pub fn new(sample_rate: f32, id: i32) -> Sampler {

        let mut voices : Vec<SamplerVoice> = Vec::new();

        for _i in 0..MAX_NOTES {
            voices.push(SamplerVoice::new(sample_rate))
        }

        // {todo} The `return` keyword is useless here.
        return Sampler {
            voices: voices,
            nb_actives_notes: 0,
            samples: Vec::new(),
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
            id: id,
            note_events: Vec::with_capacity(100),
            im_armed: false,
        };
    }
}

impl Processor for Sampler {

    fn note_on(&mut self, midi_note: u8, velocity: f32) {
        // {todo} Use an iterator instead of an index.
        for sample_idx in 0..self.samples.len() {
            if self.samples[sample_idx].apply_to_note(midi_note) {
                if self.nb_actives_notes < MAX_NOTES - 1 {
                    let note_to_active = self.nb_actives_notes as usize;
                    // {todo} Avoid using [] with the same index multiple times.
                    self.voices[note_to_active].adsr.attack = self.attack;
                    self.voices[note_to_active].adsr.decay = self.decay;
                    self.voices[note_to_active].adsr.sustain = self.sustain;
                    self.voices[note_to_active].adsr.release = self.release;
                    self.voices[note_to_active].adsr.reset();
                    self.voices[note_to_active].adsr.recalculate_rates();

                    self.voices[note_to_active].start_note(midi_note, velocity, &self.samples[sample_idx] as *const Sample);
                    self.nb_actives_notes += 1;
                    break;
                }
            }
        }
    }
   
    fn note_off(&mut self, midi_note: u8) {
        // {todo} Use an iterator instead of an index.
        for i in 0..self.voices.len() {
            if self.voices[i].note_id == midi_note && self.voices[i].active {
                self.voices[i].stop_note();
            }
        }
    }

    fn process(&mut self, outputs: *mut f32, num_samples: usize, _nb_channels: usize) {
        // {todo} Use an iterator instead of an index.
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if !self.voices[i].is_ended() {
                self.voices[i].render_next_block(outputs, num_samples);
            }
        }
        // {todo} Use an iterator instead of an index.
        for i in 0..self.nb_actives_notes {
            let i: usize = i as usize;
            if self.voices[i].is_ended() {
                self.nb_actives_notes -= 1;
                let active_notes = self.nb_actives_notes as usize;
                self.voices[i] = self.voices[active_notes];
            }
        }
    }

    fn clear_notes_events(&mut self) {
        // {todo} Use an iterator instead of an index.
        for i in 0..self.nb_actives_notes {
            self.voices[i].stop_note();
        }
        self.note_events.clear();
    }

    fn get_notes_events(&mut self) -> &Vec<MidiMessage> {
        // {todo} The `return` keyword is useless here.
        return &self.note_events;
    }

    fn add_notes_event(&mut self, midi_message: MidiMessage) {
        self.note_events.push(midi_message);
    }

    fn is_armed(&self) -> bool {
        // {todo} The `return` keyword is useless here.
        return self.im_armed;
    }

    fn set_is_armed(&mut self, is_armed: bool) {
        self.im_armed = is_armed;
    }

    fn get_id(&self) -> i32 {
        // {todo} The `return` keyword is useless here.
        return self.id;
    }

    fn add_sample(&mut self, sample: Sample) {
        self.samples.push(sample);
    }
}