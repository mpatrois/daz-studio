use crate::epiano::epiano_data::WAVES;
use crate::epiano::epiano_voice::EpianoVoice;
use crate::epiano::epiano_preset_bank::get_epiano_presets;
use crate::processor::Processor;

use crate::midimessage::NoteEvent;
use crate::preset::{Preset};

use super::epiano_preset::EpianoPreset;

const SILENCE : f32 = 0.0001; // voice choking
const MAX_VOICES: usize = 32;

#[derive(Clone)]
pub struct Epiano {
    preset_id: usize,
    muffvel: f32,
    size: i32,
    width: f32,
    poly: usize,

    fine: f32,
    random: f32,
    stretch: f32,
    sizevel: f32,
    overdrive: f32,

	treb: f32,
	tfrq: f32,
	tl: f32,
	tr: f32,

	lfo0: f32,
	lfo1: f32,
	dlfo: f32,
	lmod: f32,
	rmod : f32,

	kgrp: [KGRP; 34],

	muff: f32,
	velsens: f32,
	volume: f32,
	modwhl: f32,
	voices: [EpianoVoice; MAX_VOICES],

	i_fs: f32,

	nb_actives_notes: usize,

	presets: Vec<EpianoPreset>,

	pub note_events: Vec<NoteEvent>,
	pub waves: Vec<i16>,
}

#[derive(Copy, Clone)]

struct KGRP {
	root: i32,  //MIDI root note
	high: i32,  //highest note
	pos: i32,
	end: i32,
	loop_idx: i32,
}

impl KGRP {
	pub fn new() -> KGRP {
		KGRP {
			root: 0,
			high: 0,
			pos: 0,
			end: 0,
			loop_idx: 0,
		}
	}
}

impl Epiano {
	pub fn new(sample_rate: f32) -> Epiano {

		let presets = get_epiano_presets();

		let default_preset = presets[0].clone();

		let mut epiano = Epiano {
			size: 0,
			width: 0.,
			poly: 0,
			
			muffvel: 0.,
			sizevel: 0.,
			preset_id: 0,

			nb_actives_notes: 0,
			
			volume: 0.2,
			muff: 160.,
			tl: 0.,
			tr: 0.,
			lfo0: 0.,
			dlfo: 0.,
			lfo1: 1.0,
			modwhl: 0.,

			lmod: 0.,
			rmod: 0.,

			fine: default_preset.fine_tuning - 0.5,
			random: 0.077 * default_preset.random_tuning * default_preset.random_tuning,
			stretch: 0.,
			overdrive: 1.8 * default_preset.overdrive,
			velsens: 0.,
			tfrq: 0.,

			treb: 0.,

			kgrp: [KGRP::new(); 34],
			voices: [EpianoVoice::new(); MAX_VOICES],

			i_fs: 1. / sample_rate,

			note_events: Vec::new(),
			presets: presets,
			waves: WAVES.to_vec(),
		};


		//Waveform data and keymapping
		epiano.kgrp[ 0].root = 36;  epiano.kgrp[ 0].high = 39; //C1
		epiano.kgrp[ 3].root = 43;  epiano.kgrp[ 3].high = 45; //G1
		epiano.kgrp[ 6].root = 48;  epiano.kgrp[ 6].high = 51; //C2
		epiano.kgrp[ 9].root = 55;  epiano.kgrp[ 9].high = 57; //G2
		epiano.kgrp[12].root = 60;  epiano.kgrp[12].high = 63; //C3
		epiano.kgrp[15].root = 67;  epiano.kgrp[15].high = 69; //G3
		epiano.kgrp[18].root = 72;  epiano.kgrp[18].high = 75; //C4
		epiano.kgrp[21].root = 79;  epiano.kgrp[21].high = 81; //G4
		epiano.kgrp[24].root = 84;  epiano.kgrp[24].high = 87; //C5
		epiano.kgrp[27].root = 91;  epiano.kgrp[27].high = 93; //G5
		epiano.kgrp[30].root = 96;  epiano.kgrp[30].high =999; //C6

		epiano.kgrp[0].pos = 0;        epiano.kgrp[0].end = 8476;     epiano.kgrp[0].loop_idx = 4400;  
		epiano.kgrp[1].pos = 8477;     epiano.kgrp[1].end = 16248;    epiano.kgrp[1].loop_idx = 4903;  
		epiano.kgrp[2].pos = 16249;    epiano.kgrp[2].end = 34565;    epiano.kgrp[2].loop_idx = 6398;  
		epiano.kgrp[3].pos = 34566;    epiano.kgrp[3].end = 41384;    epiano.kgrp[3].loop_idx = 3938;  
		epiano.kgrp[4].pos = 41385;    epiano.kgrp[4].end = 45760;    epiano.kgrp[4].loop_idx = 1633; //was 1636;  
		epiano.kgrp[5].pos = 45761;    epiano.kgrp[5].end = 65211;    epiano.kgrp[5].loop_idx = 5245;  
		epiano.kgrp[6].pos = 65212;    epiano.kgrp[6].end = 72897;    epiano.kgrp[6].loop_idx = 2937;  
		epiano.kgrp[7].pos = 72898;    epiano.kgrp[7].end = 78626;    epiano.kgrp[7].loop_idx = 2203; //was 2204;  
		epiano.kgrp[8].pos = 78627;    epiano.kgrp[8].end = 100387;   epiano.kgrp[8].loop_idx = 6368;  
		epiano.kgrp[9].pos = 100388;   epiano.kgrp[9].end = 116297;   epiano.kgrp[9].loop_idx = 10452;  
		epiano.kgrp[10].pos = 116298;  epiano.kgrp[10].end = 127661;  epiano.kgrp[10].loop_idx = 5217; //was 5220; 
		epiano.kgrp[11].pos = 127662;  epiano.kgrp[11].end = 144113;  epiano.kgrp[11].loop_idx = 3099;  
		epiano.kgrp[12].pos = 144114;  epiano.kgrp[12].end = 152863;  epiano.kgrp[12].loop_idx = 4284;  
		epiano.kgrp[13].pos = 152864;  epiano.kgrp[13].end = 173107;  epiano.kgrp[13].loop_idx = 3916;  
		epiano.kgrp[14].pos = 173108;  epiano.kgrp[14].end = 192734;  epiano.kgrp[14].loop_idx = 2937;  
		epiano.kgrp[15].pos = 192735;  epiano.kgrp[15].end = 204598;  epiano.kgrp[15].loop_idx = 4732;  
		epiano.kgrp[16].pos = 204599;  epiano.kgrp[16].end = 218995;  epiano.kgrp[16].loop_idx = 4733;  
		epiano.kgrp[17].pos = 218996;  epiano.kgrp[17].end = 233801;  epiano.kgrp[17].loop_idx = 2285;  
		epiano.kgrp[18].pos = 233802;  epiano.kgrp[18].end = 248011;  epiano.kgrp[18].loop_idx = 4098;  
		epiano.kgrp[19].pos = 248012;  epiano.kgrp[19].end = 265287;  epiano.kgrp[19].loop_idx = 4099;  
		epiano.kgrp[20].pos = 265288;  epiano.kgrp[20].end = 282255;  epiano.kgrp[20].loop_idx = 3609;  
		epiano.kgrp[21].pos = 282256;  epiano.kgrp[21].end = 293776;  epiano.kgrp[21].loop_idx = 2446;  
		epiano.kgrp[22].pos = 293777;  epiano.kgrp[22].end = 312566;  epiano.kgrp[22].loop_idx = 6278;  
		epiano.kgrp[23].pos = 312567;  epiano.kgrp[23].end = 330200;  epiano.kgrp[23].loop_idx = 2283;  
		epiano.kgrp[24].pos = 330201;  epiano.kgrp[24].end = 348889;  epiano.kgrp[24].loop_idx = 2689;  
		epiano.kgrp[25].pos = 348890;  epiano.kgrp[25].end = 365675;  epiano.kgrp[25].loop_idx = 4370;  
		epiano.kgrp[26].pos = 365676;  epiano.kgrp[26].end = 383661;  epiano.kgrp[26].loop_idx = 5225;  
		epiano.kgrp[27].pos = 383662;  epiano.kgrp[27].end = 393372;  epiano.kgrp[27].loop_idx = 2811;  
		epiano.kgrp[28].pos = 383662;  epiano.kgrp[28].end = 393372;  epiano.kgrp[28].loop_idx = 2811; //ghost
		epiano.kgrp[29].pos = 393373;  epiano.kgrp[29].end = 406045;  epiano.kgrp[29].loop_idx = 4522;  
		epiano.kgrp[30].pos = 406046;  epiano.kgrp[30].end = 414486;  epiano.kgrp[30].loop_idx = 2306;  
		epiano.kgrp[31].pos = 406046;  epiano.kgrp[31].end = 414486;  epiano.kgrp[31].loop_idx = 2306; //ghost
		epiano.kgrp[32].pos = 414487;  epiano.kgrp[32].end = 422408;  epiano.kgrp[32].loop_idx = 2169;  

		//extra xfade looping...
		for k in 0..28 {
			let mut p0 = epiano.kgrp[k].end as usize;
			let mut p1 = (epiano.kgrp[k].end - epiano.kgrp[k].loop_idx) as usize;

			let mut xf = 1.0;
			let dxf = -0.02;

			while xf > 0.0 {
				epiano.waves[p0] = ((1.0 - xf) * epiano.waves[p0] as f32 + xf * epiano.waves[p1] as f32) as i16;
				p0 -= 1;
				p1 -= 1;
				xf += dxf;
			}
		}

		for v in 0..MAX_VOICES {
			epiano.voices[v].env = 0.0;
			epiano.voices[v].dec = 0.99; //all notes off
		}

		epiano.volume = 0.2;
		epiano.muff = 160.;
		epiano.tl = 0.;
		epiano.tr = 0.;
		epiano.lfo0 = 0.;
		epiano.dlfo = 0.;
		epiano.lfo1 = 1.;
		epiano.modwhl = 0.;
		
		epiano.recalculate();
		epiano
	}

	pub fn recalculate(&mut self) {
		let preset = self.presets[self.preset_id].clone();

		self.size = (12.0 * preset.hardness - 6.0) as i32;

		self.treb = 4.0 * preset.treble_boost * preset.treble_boost - 1.0; //treble gain
		if preset.treble_boost > 0.5 {
			self.tfrq = 14000.0; 
		} else {
			self.tfrq = 5000.0; //treble freq
		}
		self.tfrq = 1.0 - (-self.i_fs * self.tfrq).exp();

		self.lmod = preset.modulation + preset.modulation - 1.0; //lfo depth
		self.rmod = self.lmod;

		if preset.modulation < 0.5 {
			self.rmod = -self.rmod;
		}

		self.dlfo = 6.283 * self.i_fs * (6.22 * preset.lfo_rate - 2.61).exp(); //lfo rate

		self.velsens = 1.0 + preset.velocity_sense + preset.velocity_sense;
		if preset.velocity_sense < 0.25 {
			self.velsens -= 0.75 - 3.0 * preset.velocity_sense;
		} 

		self.width = 0.03 * preset.stereo_width;
		self.poly = 1 + (31.9 * preset.polyphony) as usize;
		self.fine = preset.fine_tuning - 0.5;
		self.random = 0.077 * preset.random_tuning * preset.random_tuning;
		self.stretch = 0.0; //0.000434f * (preset.[11] - 0.5f); parameter re-used for overdrive!
		self.overdrive = 1.8 * preset.overdrive;

		//over-ride pan/trem depth
		if self.modwhl > 0.05 {
			self.lmod = self.modwhl;
			self.rmod = self.lmod; //lfo depth
			if preset.modulation < 0.5 {
				self.rmod = -self.rmod;
			}
		}
	}
}


impl Processor for Epiano {

	fn get_name(&self) -> String { "Elec. Piano".to_string() }

    fn note_on(&mut self, midi_note: u8, velocity_origin: f32) {
        
		let preset = self.presets[self.preset_id].clone();

		let mut l = 99.0;
		let mut k : i32;
		let mut s : i32;

		let velocity = 127. * velocity_origin;

		let mut note = midi_note as i32;

		let mut current_voice = self.nb_actives_notes;

		if self.nb_actives_notes < self.poly {
			self.voices[current_voice].f0 = 0.0;
			self.voices[current_voice].f1 = 0.0;
			self.nb_actives_notes += 1;
		} else {
			for v in 0..self.poly {
				if self.voices[v].env < l { 
					l = self.voices[v].env;  
					current_voice = v;
				}
			}
		}

		k = (note - 60) * (note - 60);
		l = self.fine + self.random as f32 * (k % 13) as f32 - 6.5;  //random & fine tune
		
		if note > 60 {
			l += self.stretch * k as f32; //stretch
		}
		
		s = self.size;
		if velocity > 40. {
			s += self.sizevel as i32 * (velocity - 40.) as i32;  
		}

		k = 0;
		while note > self.kgrp[k as usize].high + s {
			k += 3;  //find keygroup
		} 
		l += (note - self.kgrp[k as usize].root) as f32; //pitch

		
		l = 32000.0 * self.i_fs * (0.05776226505 * l).exp();

		self.voices[current_voice].delta = (65536. * l) as i32;
		self.voices[current_voice].frac = 0;

		if velocity > 48. {
			k += 1; //mid velocity sample
		} 
		if velocity > 80. {
			k += 1; //high velocity sample
		} 

		self.voices[current_voice].pos = self.kgrp[k as usize].pos;
		self.voices[current_voice].end = self.kgrp[k as usize].end - 1;
		self.voices[current_voice].loop_idx = self.kgrp[k as usize].loop_idx;

		self.voices[current_voice].env = (3.0 + 2.0 * self.velsens) * f32::powf(0.0078 * velocity, self.velsens); //velocity

		if note > 60 {
			self.voices[current_voice].env *= (0.01 * (60. - note as f32)).exp(); //new! high notes quieter
		} 

		l = 50.0 + preset.modulation * preset.modulation * self.muff + self.muffvel * (velocity - 64.) as f32; //muffle
		if l < (55.0 + 0.4 * note as f32) {
			l = 55.0 + 0.4 * note as f32;
		} 
		if l > 210.0 {
			l = 210.0;
		} 
		self.voices[current_voice].ff = l * l * self.i_fs;

		self.voices[current_voice].note = note as u8; //note->pan
		if note <  12 {
			note = 12;
		} 
		if note > 108 {
			note = 108;
		}
		l = self.volume;
		self.voices[current_voice].outr = l + l * self.width * (note - 60) as f32;
		self.voices[current_voice].outl = l + l - self.voices[current_voice].outr;

		if note < 44 {
			note = 44; //limit max decay length
		} 
		self.voices[current_voice].dec = (-self.i_fs * (-1.0 + 0.03 * note as f32 - 2.0 * preset.envelope_decay).exp()).exp();
		self.voices[current_voice].note_id = midi_note;
    }
   
    fn note_off(&mut self, midi_note: u8) {
		let preset = self.presets[self.preset_id].clone();

		for v in 0..self.nb_actives_notes {
			//any voices playing that note?
			if self.voices[v].note_id == midi_note  {
				self.voices[v].dec = (-self.i_fs * (6.0 + 0.01 * midi_note as f32 - 5.0 * preset.envelope_release).exp()).exp();
			}
		}
    }

    fn all_note_off(&mut self) {
		let preset = self.presets[self.preset_id].clone();

        for i in 0..self.voices.len() {
            self.voices[i].dec = (-self.i_fs * (6.0 + 0.01 * self.voices[i].note_id as f32 - 5.0 * preset.envelope_release).exp()).exp();
        }
    }

    fn process(&mut self, outputs: &mut [f32], num_samples: usize, _nb_channels: usize) {

		let mut x;
		let mut i : i32;

		let od = self.overdrive;

		let mut idx = 0;

		if self.nb_actives_notes > 0 {
				
			for _ in 0..num_samples {
				
				let mut l = 0.0;
				let mut r = 0.0;

				for k in 0..self.nb_actives_notes {

					let mut voice = &mut self.voices[k];
					voice.frac += voice.delta;  //integer-based linear interpolation
					voice.pos += voice.frac >> 16;
					voice.frac &= 0xFFFF;
					
					if voice.pos > voice.end {
						voice.pos -= voice.loop_idx;
					}

					let pos = voice.pos as usize;
					i = self.waves[pos] as i32 + ((voice.frac * (self.waves[pos + 1] - self.waves[pos]) as i32) >> 16) as i32;
					x = voice.env * i as f32 / 32768.0;

					voice.env = voice.env * voice.dec;  //envelope

					if x > 0.0 { 
						x -= od * x * x;  
						if x < -voice.env {
							x = -voice.env; 
						}
					} 

					l += voice.outl * x;
					r += voice.outr * x;
				}

				self.tl += self.tfrq * (l - self.tl);  //treble boost
				self.tr += self.tfrq * (r - self.tr);
				r  += self.treb * (r - self.tr);
				l  += self.treb * (l - self.tl);

				self.lfo0 += self.dlfo * self.lfo1;  //LFO for tremolo and autopan
				self.lfo1 -= self.dlfo * self.lfo0;
				l += l * self.lmod * self.lfo1;
				r += r * self.rmod * self.lfo1;  //worth making all these local variables?

				outputs[idx] += l;
				outputs[idx+1] += r;

				idx += 2;
			}
		}

		for i in 0..self.nb_actives_notes {
            if self.voices[i].env < SILENCE {
                self.nb_actives_notes -= 1;
                let active_notes = self.nb_actives_notes as usize;
                self.voices[i] = self.voices[active_notes].clone();
            }
        }
    }

    fn get_current_preset_id(&self) -> usize {
        self.preset_id
    }

    fn set_current_preset_id(&mut self, id: usize) {
       if self.preset_id != id {
			self.preset_id = id;
			self.recalculate();
	   }
    }

    fn get_presets(&self) -> Vec<Box<dyn Preset>> {
        let mut presets : Vec<Box<dyn Preset>> = Vec::new();
        for preset in &self.presets {
            presets.push(Box::new(preset.clone()));
        }
        return presets;
    }

    fn prepare(&mut self, _sample_rate: f32, _num_samples: usize, _nb_channels: usize) {}
}
