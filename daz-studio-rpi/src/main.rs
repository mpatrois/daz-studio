use midir::{MidiInput, Ignore};

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 48_000.0;
const FRAMES_PER_BUFFER: u32 = 128;

mod ui;

use sequencer;
use sequencer::midimessage::MidiMessage;
use sequencer::Sequencer;
use sequencer::sequencer_data::{SequencerData, DataBroadcaster, Message};

use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::collections::HashMap;

use sdl2::keyboard::Keycode;

use core::convert::Infallible;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

fn main() {
    
    let (midi_event_sender, midi_event_receiver) = mpsc::channel::<sequencer::Message>();

    let portaudio = portaudio::PortAudio::new().unwrap();
    let mut settings = portaudio.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();
    
    settings.flags = portaudio::stream_flags::CLIP_OFF;

    let (mut data_ui, ui_sender) = SequencerData::new();

    let (mut sequencer, audio_sender) = Sequencer::new(SAMPLE_RATE as f32, FRAMES_PER_BUFFER as usize);

    // Just for testing purpose, need to synchronise this after
    data_ui.insruments = sequencer.data.insruments.clone();
    
    sequencer.audio_state_senders.push(ui_sender.clone());

    let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {

        for msg in midi_event_receiver.try_recv() {
            match msg {
                sequencer::Message::Midi(midi) => {
                    if midi.first & 0xf0 == 0x90 {
                        sequencer.note_on(midi.second);
                    } else if midi.first & 0xf0 == 0x80 {
                        sequencer.note_off(midi.second);
                    }
                },
            }
        }

        sequencer.process(buffer, frames, CHANNELS as usize);

        portaudio::Continue
    };

    let mut stream = portaudio.open_non_blocking_stream(settings, callback).unwrap();
    stream.start();

    let _connexion_midi : midir::MidiInputConnection<()>;

    let midi_in_result = MidiInput::new("midir reading input");
    if midi_in_result.is_ok() {
        let mut midi_in = midi_in_result.unwrap();
        midi_in.ignore(Ignore::None);
        let in_ports = midi_in.ports();
        
        if in_ports.len() > 0 {
            let midi_event_sender = midi_event_sender.clone();
            let conn_in = midi_in.connect(&in_ports[0], "midir-read-input", move |_stamp, message, _| {
                if message[0] & 0xf0 == 0x90 || message[0] & 0xf0 == 0x80 {
                    midi_event_sender.send(sequencer::Message::Midi( MidiMessage {
                        first: message[0],
                        second: message[1],
                        third: message[2]
                    })).unwrap();
                }
            }, ());
            if conn_in.is_ok() {
                _connexion_midi = conn_in.unwrap();
            }
        }
    }

    let broadcast = DataBroadcaster {
        senders: vec![
            audio_sender,
            ui_sender,
        ]
    };
    launch_ui(midi_event_sender, &mut data_ui, broadcast).unwrap();
}

fn launch_ui(midi_event_sender: Sender<sequencer::Message>, data_ui: &mut SequencerData, broadcaster: DataBroadcaster) -> Result<(), Infallible> {
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Daz Studio Emulator", &output_settings);

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(ui::SCREEN_WIDTH, ui::SCREEN_HEIGHT));

    let key_board_notes = HashMap::from([
        (Keycode::A, 52),
        (Keycode::Z, 53),
        (Keycode::E, 54),
        (Keycode::R, 55),
        (Keycode::T, 56),
        (Keycode::Y, 57),
        (Keycode::U, 58),
        (Keycode::I, 59),
        (Keycode::O, 60),
        (Keycode::P, 61),
        (Keycode::Q, 62),
        (Keycode::S, 63),
    ]);

    let mut main_ui = ui::MainUI  {
        metronome_left: true
    };

    'main_loop: loop {
        
        data_ui.process_messages();

        main_ui.update(data_ui, &mut display)?;
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'main_loop,
                SimulatorEvent::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main_loop;
                    }

                    let note = key_board_notes.get(&keycode); 
                    if note.is_some() {
                        midi_event_sender.send(sequencer::Message::Midi(MidiMessage {
                            first: 0x9c,
                            second: *note.unwrap(),
                            third: 127
                        })).unwrap();
                    }
                },
                SimulatorEvent::KeyUp {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    match keycode {
                        Keycode::Escape => break 'main_loop,
                        Keycode::Space => broadcaster.send(Message::PlayStop),
                        Keycode::Up => broadcaster.send(Message::PreviousInstrument),
                        Keycode::Down => broadcaster.send(Message::NextInstrument),
                        Keycode::Left => broadcaster.send(Message::PreviousPreset),
                        Keycode::Right => broadcaster.send(Message::NextPreset),
                        Keycode::W => broadcaster.send(Message::SetIsRecording(!data_ui.is_recording)),
                        Keycode::X => broadcaster.send(Message::SetMetronomeActive(!data_ui.metronome_active)),
                        Keycode::C => {
                            let new_tempo = data_ui.tempo - 1.0;
                            broadcaster.send(Message::SetTempo(new_tempo));
                        },
                        Keycode::V => {
                            let new_tempo = data_ui.tempo + 1.0;
                            broadcaster.send(Message::SetTempo(new_tempo));
                        },
                        _ => if let Some(note) = key_board_notes.get(&keycode) {
                            midi_event_sender.send(sequencer::Message::Midi(MidiMessage {
                                first: 0x8c,
                                second: *note,
                                third: 127
                            })).unwrap();
                        }
                    }
                }
                _ => {},
            }
            thread::sleep(Duration::from_millis(30));
        }
    }
    Ok(())
}
