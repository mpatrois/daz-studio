pub const NOTE_ON : u8 = 0x9c;
pub const NOTE_OFF : u8 = 0x8c; 

#[derive(Copy, Clone)]
pub struct MidiMessage {
    pub first: u8,
    pub second: u8,
    pub third: u8
}

#[derive(Copy, Clone)]
pub struct NoteEvent {
    pub note_id: u8,
    pub tick_on: i32,
    pub tick_off: i32,
    pub record_session: i32,
    pub stamp_record: i32,
}