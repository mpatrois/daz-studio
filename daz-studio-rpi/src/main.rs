use midir::{MidiInput, Ignore};

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 48_000.0;
const FRAMES_PER_BUFFER: u32 = 128;

use sequencer;
use sequencer::midimessage::MidiMessage;
use sequencer::Sequencer;
use sequencer::sequencer_data::{SequencerData, DataBroadcaster, Message};

use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::collections::HashMap;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::TextureQuery;

fn main() {
    
    let (event_sender, event_receiver) = mpsc::channel::<sequencer::Message>();

    let portaudio = portaudio::PortAudio::new().unwrap();
    let mut settings = portaudio.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();
    
    settings.flags = portaudio::stream_flags::CLIP_OFF;

    let (mut data_ui, ui_sender) = SequencerData::new();

    let (mut sequencer, audio_sender) = Sequencer::new(SAMPLE_RATE as f32, FRAMES_PER_BUFFER as usize);

    data_ui.insruments = sequencer.data.insruments.clone();

    let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {

        sequencer.process(buffer.as_mut_ptr(), frames, CHANNELS as usize);
    
        for msg in event_receiver.try_recv() {
            match msg {
                sequencer::Message::InstrumentPrev => {
                    sequencer.previous_instrument();
                },

                sequencer::Message::InstrumentNext => {
                    sequencer.next_instrument();
                },

                sequencer::Message::Midi(midi) => {
                    if midi.first & 0xf0 == 0x90 {
                        sequencer.note_on(midi.second);
                    } else if midi.first & 0xf0 == 0x80 {
                        sequencer.note_off(midi.second);
                    }
                },
            }
        }
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
            let event_sender = event_sender.clone();
            let conn_in = midi_in.connect(&in_ports[0], "midir-read-input", move |_stamp, message, _| {
                event_sender.send( sequencer::Message::Midi( MidiMessage {
                    first: message[0],
                    second: message[1],
                    third: message[2],
                    tick: 0
                })).unwrap();

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
    launch_ui(event_sender, data_ui, broadcast).unwrap();
}

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (cons_width as i32 - w) / 2;
    let cy = (cons_height as i32 - h) / 2;
  
    Rect::new(cx, cy, w as u32, h as u32)
}

fn launch_ui(event_sender: Sender<sequencer::Message>, mut data_ui: SequencerData, broadcaster: DataBroadcaster) -> Result<(), String> {
    
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

    // Load a font
    let fonts_byte = include_bytes!("../resources/fonts/AbrilFatface-Regular.ttf");
    let fonts_byte2 = include_bytes!("../resources/fonts/Roboto-Regular.ttf");
    let font = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(fonts_byte)?, 40)?;
    let font12 = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(fonts_byte2)?, 12)?;

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render("Daz Studio")
        .blended(Color::RGBA(23, 96, 118, 255))
        .map_err(|e| e.to_string())?;
    
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

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
                        event_sender.send(sequencer::Message::Midi(MidiMessage {
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
                        Keycode::Left => event_sender.send(sequencer::Message::InstrumentPrev).unwrap(),
                        Keycode::Right => event_sender.send(sequencer::Message::InstrumentNext).unwrap(),
                        Keycode::B => {
                            let new_tempo = data_ui.tempo - 1.0;
                            broadcaster.send(Message::SetTempo(new_tempo));
                        },
                        Keycode::N => {
                            let new_tempo = data_ui.tempo + 1.0;
                            broadcaster.send(Message::SetTempo(new_tempo));
                        },
                        _ => if let Some(note) = key_board_notes.get(&keycode) {
                            event_sender.send(sequencer::Message::Midi(MidiMessage {
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

        canvas.clear();
        
        let target = get_centered_rect(
            width,
            height,
            SCREEN_WIDTH,
            SCREEN_HEIGHT / 2,
        );
    
        canvas.copy(&texture, None, Some(target))?;
        
        let surface_tempo = font12
            .render(&["Tempo :", &data_ui.tempo.to_string()].join(" "))
            .blended(Color::RGBA(23, 96, 118, 255))
            .map_err(|e| e.to_string())?;
    
        let texture_tempo = texture_creator
            .create_texture_from_surface(&surface_tempo)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width: width_tempo, height: height_tempo, .. } = texture_tempo.query();
 
        canvas.copy(&texture_tempo, None, Some(Rect::new(20, 10, width_tempo, height_tempo)))?;

        let mut i = 0;
        let mut y = SCREEN_HEIGHT / 2;
        for insrument in data_ui.insruments.iter() {
            // println!("yes sir");
            let surface_intrument_name = font12
                .render(&insrument.name)
                .blended(Color::RGBA(23, 96, 118, 255))
                .map_err(|e| e.to_string())?;
    
            let texture_intrument_name = texture_creator
                .create_texture_from_surface(&surface_intrument_name)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = texture_intrument_name.query();
 
            canvas.copy(&texture_intrument_name, None, Some(Rect::new(20, (SCREEN_HEIGHT / 2) as i32 + 20 * i + 10, width, height)))?;
            i += 1;
        }
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}