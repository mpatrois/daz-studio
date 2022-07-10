use midir::{MidiInput, Ignore};

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 48_000.0;
const FRAMES_PER_BUFFER: u32 = 512;

use sequencer;
use sequencer::midimessage::MidiMessage;
use sequencer::Sequencer;
use sequencer::sequencer_data::{SequencerData, DataBroadcaster, Message};

use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::collections::HashMap;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::image::{LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::TextureQuery;

fn main() {
    
    let (midi_event_sender, midi_event_receiver) = mpsc::channel::<sequencer::Message>();

    let portaudio = portaudio::PortAudio::new().unwrap();
    let mut settings = portaudio.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();
    
    settings.flags = portaudio::stream_flags::CLIP_OFF;

    let (mut data_ui, ui_sender) = SequencerData::new();

    let (mut sequencer, audio_sender) = Sequencer::new(SAMPLE_RATE as f32, FRAMES_PER_BUFFER as usize);

    // Just for testing purpose, need to synchronise this after
    data_ui.insruments = sequencer.data.insruments.clone();

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
                        third: message[2],
                        tick: 0
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
    launch_ui(midi_event_sender, data_ui, broadcast).unwrap();
}

fn launch_ui(midi_event_sender: Sender<sequencer::Message>, mut data_ui: SequencerData, broadcaster: DataBroadcaster) -> Result<(), String> {
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Daz Studio", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(217, 219, 241));

    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();

    let logo_bytes = include_bytes!("../resources/images/daz.png");
    let texture_logo = texture_creator.load_texture_bytes(logo_bytes)?;

    // Load a font
    let fonts_byte = include_bytes!("../resources/fonts/Roboto-Regular.ttf");
    let font12px = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(fonts_byte)?, 12)?;

    let text_color = Color::RGBA(23, 96, 118, 255);
    let background_color = Color::RGB(217, 219, 241);

    let TextureQuery { width, height, .. } = texture_logo.query();

    let key_board_notes = HashMap::from([
        (sdl2::keyboard::Keycode::A, 52),
        (sdl2::keyboard::Keycode::Z, 53),
        (sdl2::keyboard::Keycode::E, 54),
        (sdl2::keyboard::Keycode::R, 55),
        (sdl2::keyboard::Keycode::T, 56),
        (sdl2::keyboard::Keycode::Y, 57),
        (sdl2::keyboard::Keycode::U, 58),
        (sdl2::keyboard::Keycode::I, 59),
        (sdl2::keyboard::Keycode::O, 60),
        (sdl2::keyboard::Keycode::P, 61),
    ]);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'running;
                    }

                    let note = key_board_notes.get(&keycode); 
                    if note.is_some() {
                        midi_event_sender.send(sequencer::Message::Midi(MidiMessage {
                            first: 0x9c,
                            second: *note.unwrap(),
                            third: 127,
                            tick: 0
                        })).unwrap();
                    }
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {

                    match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::Up => broadcaster.send(Message::PreviousInstrument),
                        Keycode::Down => broadcaster.send(Message::NextInstrument),
                        Keycode::Left => broadcaster.send(Message::PreviousPreset),
                        Keycode::Right => broadcaster.send(Message::NextPreset),
                        Keycode::V => broadcaster.send(Message::SetMetronomeActive(!data_ui.metronome_active)),
                        Keycode::C => broadcaster.send(Message::SetIsRecording(!data_ui.is_recording)),
                        Keycode::B => {
                            let new_tempo = data_ui.tempo - 1.0;
                            broadcaster.send(Message::SetTempo(new_tempo));
                        },
                        Keycode::N => {
                            let new_tempo = data_ui.tempo + 1.0;
                            broadcaster.send(Message::SetTempo(new_tempo));
                        },
                        _ => if let Some(note) = key_board_notes.get(&keycode) {
                            midi_event_sender.send(sequencer::Message::Midi(MidiMessage {
                                first: 0x8c,
                                second: *note,
                                third: 127,
                                tick: 0
                            })).unwrap();
                        }
                    }
                }
                _ => {},
            }   
        }

        data_ui.process_messages();

        canvas.set_draw_color(Color::RGB(217, 219, 241));
        canvas.clear();
        
        canvas.copy(&texture_logo, None, Some(Rect::new(SCREEN_WIDTH as i32/2, SCREEN_HEIGHT  as i32 / 2 - (height as i32/3) / 2, width/3, height/3)))?;
        
        // Tempo
        {
            let surface = font12px
                .render(&["BPM :", &data_ui.tempo.to_string()].join(" "))
                .blended(text_color)
                .map_err(|e| e.to_string())?;
        
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
    
            let TextureQuery { width: width_tempo, height: height_tempo, .. } = texture.query();
            canvas.copy(&texture, None, Some(Rect::new(20, 10, width_tempo, height_tempo)))?;
        }

        // Metronome
        {
            let surface = font12px
                .render(&["Metronome :", &data_ui.metronome_active.to_string()].join(" "))
                .blended(text_color)
                .map_err(|e| e.to_string())?;
        
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
    
            let TextureQuery { width: width_tempo, height: height_tempo, .. } = texture.query();
            canvas.copy(&texture, None, Some(Rect::new(20, 25, width_tempo, height_tempo)))?;
        }
        
        // Recording
        {
            let surface = font12px
                .render(&["Recording :", &data_ui.is_recording.to_string()].join(" "))
                .blended(text_color)
                .map_err(|e| e.to_string())?;
        
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
    
            let TextureQuery { width: width_tempo, height: height_tempo, .. } = texture.query();
            canvas.copy(&texture, None, Some(Rect::new(20, 40, width_tempo, height_tempo)))?;
        }

        let mut i : i32 = 0;
        let mut y = 100;
        let height_rect = 30;
        for insrument in data_ui.insruments.iter() {

            let mut color_name = text_color;
            if data_ui.instrument_selected_id == i as usize {
                color_name = background_color;
            }

            let surface_intrument_name = font12px
                .render(&insrument.name)
                .blended(color_name)
                .map_err(|e| e.to_string())?;
    
            let texture_intrument_name = texture_creator
                .create_texture_from_surface(&surface_intrument_name)
                .map_err(|e| e.to_string())?;
            
            let surface_intrument_preset = font12px
                .render(&insrument.presets[insrument.current_preset_id])
                .blended(color_name)
                .map_err(|e| e.to_string())?;
    
            let texture_intrument_preset = texture_creator
                .create_texture_from_surface(&surface_intrument_preset)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = texture_intrument_name.query();
            let TextureQuery { width: width_preset, height: height_preset, .. } = texture_intrument_preset.query();
            
            canvas.set_draw_color(text_color);

            if data_ui.instrument_selected_id as i32 == i {
                canvas.fill_rect(Rect::new(20, y, 100, height_rect))?;
                canvas.set_draw_color(Color::RGB(217, 219, 241));
            } else {
                canvas.draw_rect(Rect::new(20, y, 100, height_rect))?;
            }

            canvas.copy(&texture_intrument_name, None, Some(Rect::new(20, y, width, height)))?;
            canvas.copy(&texture_intrument_preset, None, Some(Rect::new(20, y + height as i32, width_preset, height_preset)))?;

            i += 1;
            y += height_rect as i32 + 5
        }
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}