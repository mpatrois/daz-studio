mod Data {
    enum Message {
      SetTempo(f32),
      SetVolume(f32),
      SetMetronomeActive(bool),
      SetIsRecording(bool),
      SetCurrentInstrumentSelected(use),
    }
  
    struct Data {
      tempo: f32,
      volume: f32,
      metronome_active: bool,
      is_recording: bool,
      instrument_selected_id: bool,
      receiver: std::sync::mpsc::Receiver<Message>,
    }
  
    impl Data {
      fn process_messages(&mut self) {
        for msg in self.receiver.try_iter() {
          match msg {
            Some(Message::SetTempo(x)) => self.tempo = x,
            _ => (),
          }
        }
      }
    }
  }