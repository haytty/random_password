use std::fmt::{Display, Formatter};
use crate::char_pattern;

pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn random_password(length: u8, uppercase: bool, lowercase: bool, symbol: bool, digit: bool) -> Password {
    let mut rng = rand::thread_rng();
    let s: String = char_pattern::random_chars(
        length, uppercase, lowercase, symbol, digit, &mut rng
    ).iter().collect();
   
    Password::new(s)
}
