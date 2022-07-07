use std::sync::mpsc;

#[derive(Copy, Clone)]
pub enum Message {
    SetTempo(f32),
    SetVolume(f32),
    SetMetronomeActive(bool),
    SetIsRecording(bool),
    SetCurrentInstrumentSelected(usize),
}

#[derive(Clone)]
pub struct DataBroadcaster {
    pub senders: Vec<mpsc::Sender<Message>>
}

impl DataBroadcaster {
    pub fn send(&self, msg: Message) {
        for sender in self.senders.iter() {
            sender.send(msg.clone()).unwrap();
        }
    }
}

pub struct Data {
    pub tempo: f32,
    pub tick_time: f32,
    pub volume: f32,
    pub metronome_active: bool,
    pub ticks_per_quarter_note: i32,
    pub is_recording: bool,
    pub instrument_selected_id: usize,
    pub receiver: mpsc::Receiver<Message>,
}

impl Data {
    
    pub fn new() -> (Data, mpsc::Sender<Message>) {
        let (sender, receiver) = mpsc::channel::<Message>();
        let mut data = Data {
            tempo: 90.0,
            volume: 0.6,
            metronome_active: true,
            is_recording: false,
            ticks_per_quarter_note: 960,
            instrument_selected_id: 0,
            tick_time: 0.0,
            receiver
        };
        data.compute_tick_time();
        (data, sender)
    }

    pub fn process_messages(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                Message::SetTempo(x) => {
                    self.tempo = x;
                    self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
                    // scompute_tick_time
                },
                _ => (),
            }
        }
    }

    fn compute_tick_time(&mut self) {
        self.tick_time = (60.0 / self.tempo) / self.ticks_per_quarter_note as f32;
    }
}