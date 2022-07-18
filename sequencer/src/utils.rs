pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub fn midi_note_fo_hertz(midi_note: u8) -> f32 {
    let a = 440.;
    (a / 32.) * f32::powf(2., (midi_note as f32 - 9.) / 12.0)
}