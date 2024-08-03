use crate::random::Random;

pub const UPPERCASE_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G',
    'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z'
];

pub struct Uppercase;

impl Random for Uppercase {
    fn available_values() -> &'static [char] {
        &UPPERCASE_CHARS
    }
}
