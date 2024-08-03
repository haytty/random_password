use crate::random::Random;

pub const DIGITS: [char; 10] = [
    '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9'
];

pub struct Digit;

impl Random for Digit {
    fn available_values() -> &'static [char] {
        &DIGITS
    }
}