use crate::random::Random;

pub const SYMBOLS: [char; 17] = [
    '!', '#', '$', '%', '&', '.',
    '(', ')', '*', '+', '-',
    '/', ':', ';', '?', '@', '_'
];

pub struct Symbol;

impl Random for Symbol {
    fn available_values() -> &'static [char] {
        &SYMBOLS
    }
}
