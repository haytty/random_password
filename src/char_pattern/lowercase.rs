use crate::random::Random;

pub const LOWERCASE_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z'
];

pub struct Lowercase;

impl Random for Lowercase {
    fn available_values() -> &'static [char] {
        &LOWERCASE_CHARS
    }
}