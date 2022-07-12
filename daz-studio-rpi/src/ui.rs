
use sequencer::sequencer_data::SequencerData;

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

pub struct MainUI {
   pub metronome_left: bool,
}

impl MainUI {
    pub fn update(&mut self,
        data_ui: &mut SequencerData,
        display: &mut SimulatorDisplay<Rgb888>,
    ) -> Result<(), Infallible> {
    
        let instrument_color = Rgb888::new(234, 224, 213);
        let metronome_color = BACKGROUND_COLOR;
        let play_head_color = Rgb888::new(254, 177, 4);
        let play_color = Rgb888::new(53, 114, 102);
        let record_color = Rgb888::new(255, 51, 36);

        if data_ui.bpm_has_biped {
            self.metronome_left = !self.metronome_left;
        }
        data_ui.bpm_has_biped = false;
    
        let fill_rect = PrimitiveStyleBuilder::new()
            .fill_color(instrument_color)
            .build();
    
        let stroke_rect = PrimitiveStyleBuilder::new()
            .stroke_color(instrument_color)
            .stroke_width(1)
            .build();
    
        display.clear(BACKGROUND_COLOR)?;
    
        let header_rectangle = Rectangle::new(
            Point::new(0, 0),
            Size::new(SCREEN_WIDTH, 25)
        );
        let left_margin = 10;
    
        header_rectangle.into_styled(fill_rect).draw(display)?;
    
        let margin_right = 8;
        let h_triangle = 15;
        let w_triangle = h_triangle - 3;
    
        let circle_record_x = left_margin + w_triangle + margin_right;
        let triangle_metronome_x = circle_record_x + h_triangle + margin_right;
    
        // Play
        {
            let triangle_play = Triangle::new(
                Point::new(left_margin, header_rectangle.center().y - h_triangle / 2),
                Point::new(left_margin + w_triangle, header_rectangle.center().y),
                Point::new(left_margin, header_rectangle.center().y + h_triangle / 2),
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
            let triangle_metronome = Triangle::new(
                Point::new(triangle_metronome_x, header_rectangle.center().y + h_triangle / 2),
                Point::new(triangle_metronome_x + w_triangle / 2, header_rectangle.center().y - h_triangle / 2),
                Point::new(triangle_metronome_x + w_triangle, header_rectangle.center().y + h_triangle / 2),
            );
            if data_ui.metronome_active {
                triangle_metronome
                    .into_styled(PrimitiveStyle::with_fill(metronome_color))
                    .draw(display)?;
            } else {
                triangle_metronome
                    .into_styled(PrimitiveStyle::with_stroke(metronome_color, 1))
                    .draw(display)?;
            }
    
            let mut point_head_metronome : Point;
            if self.metronome_left {
                point_head_metronome = Point::new(triangle_metronome_x, header_rectangle.center().y - h_triangle / 2);
            } else {
                point_head_metronome = Point::new(triangle_metronome_x + w_triangle, header_rectangle.center().y - h_triangle / 2);
            }
    
            let circle_metronome = Circle::with_center(
                point_head_metronome, 
                4 as u32
            );
    
            circle_metronome
                .into_styled(PrimitiveStyle::with_fill(metronome_color))
                .draw(display)?;
    
            let line_metronome = Line::new(
                Point::new(triangle_metronome_x + w_triangle / 2, header_rectangle.center().y + h_triangle / 2 - 4),
                point_head_metronome
            );
            line_metronome
                .into_styled(PrimitiveStyle::with_stroke(metronome_color, 1))
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
    
        // Instruments
        {
            let margin_top_instrument = 10;
            let mut i : i32 = 0;
            let mut y = header_rectangle.bottom_right().unwrap().y + margin_top_instrument;
            let height_rect = 30;
            for insrument in data_ui.insruments.iter() {
    
                let mut text_style = MonoTextStyle::new(&FONT_6X12, instrument_color);
    
                if data_ui.instrument_selected_id == i as usize {
                    text_style = MonoTextStyle::new(&FONT_6X12, BACKGROUND_COLOR);
                }
    
                let rectangle_instrument_name = Rectangle::new(
                    Point::new(left_margin, y), 
                    Size::new(100, height_rect)
                );
                
                let rectangle_instrument_notes = Rectangle::new(
                    Point::new(100 + left_margin + left_margin, y), 
                    Size::new(SCREEN_WIDTH - 100 - (left_margin as u32 * 3), height_rect)
                );
                rectangle_instrument_notes
                    .into_styled(stroke_rect)
                    .draw(display)?;
    
                if data_ui.instrument_selected_id as i32 == i {
                    rectangle_instrument_name
                        .into_styled(fill_rect)
                        .draw(display)?;
                } else {
                    rectangle_instrument_name
                        .into_styled(stroke_rect)
                        .draw(display)?;
                }
    
                let x = rectangle_instrument_name.top_left.x + 10;
                Text::new(&insrument.name, Point::new(x, y + 6 + 4), text_style).draw(display)?;
                Text::new(&insrument.presets[insrument.current_preset_id], Point::new(x, y + (10 + 2) * 2), text_style).draw(display)?;
    
                i += 1;
                y += height_rect as i32 + margin_top_instrument;
    
    
                let tick_width : f32 = rectangle_instrument_notes.size.width as f32 / (data_ui.bars as f32 * 4. * data_ui.ticks_per_quarter_note as f32);
                let tick_x =  (tick_width * data_ui.tick as f32) as i32;
                
                let play_head = Line::new(
                    Point::new(rectangle_instrument_notes.top_left.x + tick_x, rectangle_instrument_notes.top_left.y),
                    Point::new(rectangle_instrument_notes.top_left.x + tick_x, rectangle_instrument_notes.bottom_right().unwrap().y)
                );
                play_head
                    .into_styled(PrimitiveStyle::with_stroke(play_head_color, 1))
                    .draw(display)?;
    
            }
        }
    
        Ok(())
    }   
}