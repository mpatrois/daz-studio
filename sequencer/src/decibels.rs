const MINUS_INFINITY_DB: f32 = -100.0;

pub fn db_to_gain (decibels: f32) -> f32
{
    if decibels > MINUS_INFINITY_DB {
        return f32::powf(10.0, decibels * 0.05);
    }
    // {todo} The `return` keyword is useless here. The first `return`
    // is required but we can avoid it by using an `else` branch.
    return 0.0;
}