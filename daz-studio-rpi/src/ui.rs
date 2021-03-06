
use sequencer::{sequencer_data::{SequencerData, InstrumentData}, midimessage::NoteEvent};

use core::convert::Infallible;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, 
        PrimitiveStyle, 
        PrimitiveStyleBuilder, 
        Rectangle, 
        Triangle,
        Line, 
    },
    text::Text,
};
use embedded_graphics_simulator::{
    SimulatorDisplay
};

pub const SCREEN_WIDTH: u32 = 320;
pub const SCREEN_HEIGHT: u32 = 240;

pub const BACKGROUND_COLOR : Rgb888 = Rgb888::new(34, 51, 59);

pub const INSTRUMENT_COLOR : Rgb888 = Rgb888::new(234, 224, 213);
pub const WAVEFORM_COLOR : Rgb888 = Rgb888::new(34, 51, 59);

pub const LEFT_MARGIN : i32 = 10;
pub const HEIGHT_RECT_INSTRU : i32 = 30;


pub struct MainUI {
   pub metronome_left: bool,
}

impl MainUI {

    pub fn draw_wave_form(
        &mut self, 
        data_ui: &mut SequencerData, 
        display: &mut SimulatorDisplay<Rgb888>, 
        box_draw: Rectangle)  -> Result<(), Infallible> {

        let fill_line = PrimitiveStyleBuilder::new()
            .stroke_color(WAVEFORM_COLOR)
            .stroke_width(1)
            .build();

        if data_ui.audio_wave_form.len() > 0 {
            let size_sample = box_draw.size.width as f32 / data_ui.audio_wave_form.len() as f32;

            let half_box = box_draw.size.height / 2;
            
            let mut i = 0;
            for _ in 0..data_ui.audio_wave_form.len() / 2 {

                let x1 = box_draw.top_left.x + (i as f32 * size_sample) as i32;
                let x2 = box_draw.top_left.x + ((i+2) as f32 * size_sample) as i32;
                let audio1 = half_box as f32 - data_ui.audio_wave_form[i] * box_draw.size.height as f32;
                let audio2 = half_box as f32 - data_ui.audio_wave_form[i] * box_draw.size.height as f32;

                Line::new(
                    Point::new(x1, audio1 as i32),
                    Point::new(x2, audio2 as i32)
                ).into_styled(fill_line)
                .draw(display)?;

                i += 2;
            }
        }
        Ok({})
    }

    pub fn update(&mut self,
        data_ui: &mut SequencerData,
        display: &mut SimulatorDisplay<Rgb888>,
    ) -> Result<(), Infallible> {

        let metronome_color_active = Rgb888::new(15, 113, 214);
        let metronome_color = BACKGROUND_COLOR;
        let play_color = Rgb888::new(53, 114, 102);
        let record_color = Rgb888::new(255, 51, 36);

        if data_ui.bpm_has_biped {
            self.metronome_left = !self.metronome_left;
        }
        data_ui.bpm_has_biped = false;
    
        let fill_rect = PrimitiveStyleBuilder::new()
            .fill_color(INSTRUMENT_COLOR)
            .build();
    
        display.clear(BACKGROUND_COLOR)?;
    
        let header_rectangle = Rectangle::new(
            Point::new(0, 0),
            Size::new(SCREEN_WIDTH, 25)
        );
    
        header_rectangle.into_styled(fill_rect).draw(display)?;
    
        let margin_right = 8;
        let h_triangle = 15;
        let w_triangle = h_triangle - 3;
    
        let circle_record_x = LEFT_MARGIN + w_triangle + margin_right;
        let triangle_metronome_x = circle_record_x + h_triangle + margin_right;
    
        // Play
        {
            let triangle_play = Triangle::new(
                Point::new(LEFT_MARGIN, header_rectangle.center().y - h_triangle / 2),
                Point::new(LEFT_MARGIN + w_triangle, header_rectangle.center().y),
                Point::new(LEFT_MARGIN, header_rectangle.center().y + h_triangle / 2),
            );
            if data_ui.is_playing {
                let style_triangle = PrimitiveStyle::with_fill(play_color);
                triangle_play
                    .into_styled(style_triangle)
                    .draw(display)?;
            } else {
                let style_triangle = PrimitiveStyle::with_stroke(play_color, 1);
                triangle_play
                    .into_styled(style_triangle)
                    .draw(display)?;
            }
        }
    
        // Record
        {
            let circle_record = Circle::new(
                Point::new(circle_record_x, header_rectangle.center().y - h_triangle / 2), 
                h_triangle as u32
            );
            if data_ui.is_recording {
                circle_record
                    .into_styled(PrimitiveStyle::with_fill(record_color))
                    .draw(display)?;
            } else {
                circle_record
                    .into_styled(PrimitiveStyle::with_stroke(record_color, 1))
                    .draw(display)?;
            }
        }
    
        // Metronome
        {
            let mut color = metronome_color;
            if data_ui.metronome_active {
                color = metronome_color_active;
            }
            let triangle_metronome = Triangle::new(
                Point::new(triangle_metronome_x, header_rectangle.center().y + h_triangle / 2),
                Point::new(triangle_metronome_x + w_triangle / 2, header_rectangle.center().y - h_triangle / 2),
                Point::new(triangle_metronome_x + w_triangle, header_rectangle.center().y + h_triangle / 2),
            );
            triangle_metronome
                .into_styled(PrimitiveStyle::with_stroke(color, 1))
                .draw(display)?;
    
            let point_head_metronome : Point;
            if self.metronome_left {
                point_head_metronome = Point::new(triangle_metronome_x, header_rectangle.center().y - h_triangle / 2);
            } else {
                point_head_metronome = Point::new(triangle_metronome_x + w_triangle, header_rectangle.center().y - h_triangle / 2);
            }
    
            let circle_metronome = Circle::with_center(
                point_head_metronome, 
                4 as u32
            );
    
            let line_metronome = Line::new(
                Point::new(triangle_metronome_x + w_triangle / 2, header_rectangle.center().y + h_triangle / 2 - 4),
                point_head_metronome
            );

            circle_metronome
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(display)?;
            line_metronome
                .into_styled(PrimitiveStyle::with_stroke(color, 1))
                .draw(display)?;

        }
    
        // Tempo
        {
            let text_style = MonoTextStyle::new(&FONT_8X13, BACKGROUND_COLOR);
            
            let text_data =  ["BPM", &data_ui.tempo.to_string()].join(":").to_string();
            let text = Text::new(
                &text_data, 
                Point::new(SCREEN_WIDTH as i32 - 48 - 10, header_rectangle.center().y + 13/3), 
                text_style
            );
                
            text.draw(display)?;
        }
        
        // Quantize
        {
            let text_style = MonoTextStyle::new(&FONT_8X13, BACKGROUND_COLOR);
            
            let text_data =  ["Q", &data_ui.get_quantize().to_string()].join(":").to_string();
            let text = Text::new(
                &text_data, 
                Point::new(SCREEN_WIDTH as i32 - 48 - 10 - 45, header_rectangle.center().y + 13/3), 
                text_style
            );
                
            text.draw(display)?;
        }
    
        // Instruments
        {
            let margin_top_instrument = 10;
            let mut i : usize = 0;
            let mut y = header_rectangle.bottom_right().unwrap().y + margin_top_instrument;

            let instrument_container = Rectangle::new(
                Point::new(LEFT_MARGIN, y), 
                Size::new(
                    SCREEN_WIDTH - LEFT_MARGIN as u32 * 2, 
                    SCREEN_HEIGHT - header_rectangle.size.height - margin_top_instrument as u32 * 2
                )
            );
            
            let max_instrument_per_page : usize = instrument_container.size.height as usize / (HEIGHT_RECT_INSTRU + margin_top_instrument) as usize;

            let mut offset = 0;

            let index_instrument_selected =  data_ui.instrument_selected_id;
            let nb_instrument = data_ui.instruments.len();

            if index_instrument_selected > max_instrument_per_page / 2 && max_instrument_per_page < data_ui.instruments.len() {
                offset = index_instrument_selected - max_instrument_per_page / 2;
                if index_instrument_selected + 1 >= (nb_instrument - max_instrument_per_page / 2) {
                    offset = nb_instrument - max_instrument_per_page - 1;
                }
                y -= offset as i32 * (HEIGHT_RECT_INSTRU + margin_top_instrument);
            }

            for instrument in data_ui.instruments.iter() {
                if i >= offset && i <= offset + max_instrument_per_page {
                    self.draw_instument(
                        data_ui,
                        instrument,
                        display,
                        y,
                        i
                    )?;
                }
                i += 1;
                y += HEIGHT_RECT_INSTRU as i32 + margin_top_instrument;   
            }

            if max_instrument_per_page + 1 < data_ui.instruments.len() {
                let size_scroll_bar = 3;
                let size_view_bar = (instrument_container.size.height) as f32 * ((max_instrument_per_page + 1) as f32 * 1.0 / nb_instrument as f32);
                
                Rectangle::new(
                    Point::new(
                        instrument_container.bottom_right().unwrap().x + 3,
                        (instrument_container.top_left.y as f32 + offset as f32 * instrument_container.size.height as f32 * (1.0 / nb_instrument as f32)) as i32,
                    ), 
                    Size::new(
                        size_scroll_bar,
                        size_view_bar as u32
                    )
                ).into_styled(fill_rect)
                .draw(display)?;
            }
        }

        self.draw_wave_form(data_ui, display, Rectangle::new(Point::new(SCREEN_WIDTH as i32 / 2 - 50 / 2, 0), Size::new(50, 30)))?;
    
        Ok(())
    }

    pub fn draw_instument(&mut self,
        data_ui: &SequencerData,
        instrument: &InstrumentData,
        display: &mut SimulatorDisplay<Rgb888>,
        y: i32,
        i: usize
    ) -> Result<(), Infallible> {

        let play_head_color = Rgb888::new(254, 177, 4);

        let stroke_rect = PrimitiveStyleBuilder::new()
            .stroke_color(INSTRUMENT_COLOR)
            .stroke_width(1)
            .build();

        let fill_rect = PrimitiveStyleBuilder::new()
            .fill_color(INSTRUMENT_COLOR)
            .build();
        
        let width_box_name = 90;

        let mut text_style = MonoTextStyle::new(&FONT_6X12, INSTRUMENT_COLOR);
    
        if data_ui.instrument_selected_id == i as usize {
            text_style = MonoTextStyle::new(&FONT_6X12, BACKGROUND_COLOR);
        }

        let rectangle_instrument_name = Rectangle::new(
            Point::new(LEFT_MARGIN, y), 
            Size::new(width_box_name, HEIGHT_RECT_INSTRU as u32)
        );

        let height_rms_left = (HEIGHT_RECT_INSTRU as f32 * instrument.rms_left) as i32;
        let height_rms_right = (HEIGHT_RECT_INSTRU as f32 * instrument.rms_right) as i32;
        let size_one_rms = 4;
        let magin_left_rms = 2;
        
        let right_box_name = LEFT_MARGIN + width_box_name as i32;

        let rms_rect_left_container = Rectangle::new(
            Point::new(right_box_name + magin_left_rms, y), 
            Size::new(size_one_rms, HEIGHT_RECT_INSTRU as u32)
        );
        
        let rms_rect_left = Rectangle::new(
            Point::new(right_box_name + magin_left_rms, y + (HEIGHT_RECT_INSTRU as i32 - height_rms_left) as i32), 
            Size::new(size_one_rms, height_rms_left as u32)
        );
        
        let rms_rect_right_container = Rectangle::new(
            Point::new(right_box_name + (magin_left_rms * 2) + size_one_rms as i32, y), 
            Size::new(size_one_rms, HEIGHT_RECT_INSTRU as u32)
        );

        let rms_rect_right = Rectangle::new(
            Point::new(right_box_name + (magin_left_rms * 2) + size_one_rms as i32, y + (HEIGHT_RECT_INSTRU as i32 - height_rms_right)), 
            Size::new(size_one_rms, height_rms_right as u32)
        );

        rms_rect_left_container
            .into_styled(stroke_rect)
            .draw(display)?;
        
        rms_rect_left
            .into_styled(fill_rect)
            .draw(display)?;
        
        rms_rect_right_container
            .into_styled(stroke_rect)
            .draw(display)?;
        
        rms_rect_right
            .into_styled(fill_rect)
            .draw(display)?;
        
        let rectangle_instrument_notes = Rectangle::new(
            Point::new(
                right_box_name + (2 * (size_one_rms + magin_left_rms as u32) as i32 + magin_left_rms + LEFT_MARGIN / 2), 
                y
            ), 
            Size::new(SCREEN_WIDTH - width_box_name - (LEFT_MARGIN as u32 * 4), HEIGHT_RECT_INSTRU as u32)
        );
        rectangle_instrument_notes
            .into_styled(stroke_rect)
            .draw(display)?;

        if data_ui.instrument_selected_id == i {
            rectangle_instrument_name
                .into_styled(fill_rect)
                .draw(display)?;
        } else {
            rectangle_instrument_name
                .into_styled(stroke_rect)
                .draw(display)?;
        }

        let x = rectangle_instrument_name.top_left.x + 10;
        Text::new(&instrument.name, Point::new(x, y + 6 + 4), text_style).draw(display)?;
        Text::new(&instrument.presets[instrument.current_preset_id], Point::new(x, y + (10 + 2) * 2), text_style).draw(display)?;

        let tick_width : f32 = rectangle_instrument_notes.size.width as f32 / (data_ui.bars as f32 * 4. * data_ui.ticks_per_quarter_note as f32);
        let tick_x =  (tick_width * data_ui.tick as f32) as i32;
        
        let play_head = Line::new(
            Point::new(rectangle_instrument_notes.top_left.x + tick_x, rectangle_instrument_notes.top_left.y),
            Point::new(rectangle_instrument_notes.top_left.x + tick_x, rectangle_instrument_notes.bottom_right().unwrap().y)
        );
        play_head
            .into_styled(PrimitiveStyle::with_stroke(play_head_color, 1))
            .draw(display)?;

        self.draw_notes(
            display, 
            &instrument.paired_notes, 
            rectangle_instrument_notes, 
            data_ui
        )?;

        Ok({})
    }

    fn draw_notes(&mut self, display: &mut SimulatorDisplay<Rgb888>, note_events: &Vec<NoteEvent>, box_draw: Rectangle, data_ui: & SequencerData) -> Result<(), Infallible> {
        
        let nb_ticks = data_ui.bars * 4 * data_ui.ticks_per_quarter_note;
        let mut max_note = 0;
        let mut min_note = 108;

        let fill_rect = PrimitiveStyleBuilder::new()
            .fill_color(INSTRUMENT_COLOR)
            .build();
    
        for note_event in note_events.iter() {
            if max_note < note_event.note_id {
                max_note = note_event.note_id;
            }
            if min_note > note_event.note_id {
                min_note = note_event.note_id;
            }
        }
    
        let size_tick = box_draw.size.width as f32 * 1.0 / nb_ticks as f32;
    
        for note_event in note_events.iter() {
            let note_index = (max_note - note_event.note_id) as i32;

            let tick_duration : i32;
            if note_event.tick_off == -1 {
                tick_duration = data_ui.tick - note_event.tick_on;
            } else {
                tick_duration = note_event.tick_off + 1 - note_event.tick_on;
            } 
            
            let x_note = box_draw.top_left.x + (note_event.tick_on as f32 * size_tick) as i32;
            let h = box_draw.size.height / ((max_note as u32 + 2) - min_note as u32);
            let y_note = box_draw.top_left.y + note_index as i32 * h as i32 + box_draw.size.height as i32 / 2 - ((max_note as i32 - min_note as i32) * h as i32) / 2;
            let mut w_note = (tick_duration as f32 * size_tick as f32) as u32;
    
            if w_note < 4 {
                w_note = 4;
            }

            Rectangle::new(
                Point::new(x_note, y_note), 
                Size::new(w_note as u32, 2)
            ).into_styled(
                fill_rect
            ).draw(display)?;
        }
        Ok({})    
    }

}