// {todo} Maybe use an `enum` like this instead:
//
// https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-fieldless-enumerations
pub const NOTE_ON : u8 = 0x9c;
pub const NOTE_OFF : u8 = 0x8c; 

#[derive(Copy, Clone)]
pub struct MidiMessage {
    // {todo} Try to avoid `pub` everywhere and use methods for
    // encapsulation.
    pub first: u8,
    pub second: u8,
    pub third: u8,
    pub tick: i32
}