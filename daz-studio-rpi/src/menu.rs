use sequencer::{sequencer_data::{SequencerData, InstrumentData, Message ,DataBroadcaster}};


use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;

pub struct MenuItem {
    pub name: String,
    pub value: String,
}

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub current: usize,
    pub is_opened: bool,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            current: 0,
            is_opened: false,
            items: vec![
                MenuItem {
                    name: "Open Project".to_string(),
                    value: ">".to_string(),
                },
                MenuItem {
                    name: "Save".to_string(),
                    value: ">".to_string(),
                },
                MenuItem {
                    name: "Save as".to_string(),
                    value: ">".to_string(),
                },
            ]
        }
    }

    pub fn up(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        } else {
            self.current = self.items.len() - 1;
        }
    }

    pub fn down(&mut self) {
        self.current += 1;
        if self.current > self.items.len() - 1 {
            self.current = 0;
        }
    }
    
    pub fn enter(&mut self, data_ui: &mut SequencerData,  broadcaster: &DataBroadcaster) {
        if self.current == 1 {
            data_ui.export_to_file("./saves/test.daz".to_string()).unwrap();
            self.is_opened = false;
        } else if self.current == 0 {
            let instruments = data_ui.import_from_file("./saves/test.daz".to_string()).unwrap();
            broadcaster.send(Message::SetInstruments(instruments));
            self.is_opened = false;
        }
    }

}