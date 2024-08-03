use std::cmp::min;
use std::collections::HashMap;
use rand::prelude::{SliceRandom};
use rand::Rng;
use crate::char_pattern::digit::Digit;
use crate::char_pattern::lowercase::Lowercase;
use crate::char_pattern::symbol::Symbol;
use crate::char_pattern::uppercase::Uppercase;
use crate::random::Random;

pub mod uppercase;
pub mod lowercase;
pub mod symbol;
pub mod digit;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum CharPattern {
    Digit,
    Lowercase,
    Uppercase,
    Symbol,
}

type Length = usize;

fn generate_random_vec<R>(length: Length, wants: u8, rng: &mut R) -> Vec<Length>
where
    R: Rng + ?Sized,
{
    fn create_random_vec<R>(mut acc: Vec<Length>, length: Length, wants: u8, max_length: Length, rng: &mut R) -> Vec<Length>
    where
        R: Rng + ?Sized,
    {
        match wants {
            0 => acc,
            1 => {
                acc.push(length);
                create_random_vec(acc, length - length, wants - 1, max_length, rng)
            }
            _ => {
                let l = min(length, max_length);
                let number = rng.gen_range(0..=l);
                acc.push(number);

                create_random_vec(acc, length - number, wants - 1, max_length, rng)
            }
        }
    }

    create_random_vec(vec![], length, wants, length / 2, rng)
}

fn random_char_pattern_length<R>(patterns: Vec<CharPattern>, length: Length, rng: &mut R) -> HashMap<CharPattern, Length>
where
    R: Rng + ?Sized,
{
    let mut random_length_vec = generate_random_vec(length as Length, patterns.len() as u8, rng);
    random_length_vec.shuffle(rng);
    random_length_vec.shuffle(rng);

    let char_pattern_length_map: HashMap<CharPattern, Length> =
        patterns.iter().zip(random_length_vec.iter()).map(|(&pattern, &length)| {
            (pattern, length)
        }).into_iter().collect();

    char_pattern_length_map
}

fn get_char_patterns(uppercase: bool, lowercase: bool, symbol: bool, digit: bool) -> Vec<CharPattern> {
    match (uppercase, lowercase, symbol, digit) {
        (false, false, false, false) => {
            vec![
                CharPattern::Uppercase,
                CharPattern::Lowercase,
                CharPattern::Symbol,
                CharPattern::Digit,
            ]
        }
        _ => {
            let mut char_patterns = Vec::new();
            if uppercase {
                char_patterns.push(CharPattern::Uppercase);
            }
            if lowercase {
                char_patterns.push(CharPattern::Lowercase);
            }
            if symbol {
                char_patterns.push(CharPattern::Symbol);
            }
            if digit {
                char_patterns.push(CharPattern::Digit);
            }
            char_patterns
        }
    }
}

pub fn random_chars<R>(length: u8, uppercase: bool, lowercase: bool, symbol: bool, digit: bool, rng: &mut R) -> Vec<char>
where
    R: Rng + ?Sized,
{
    let char_patterns = get_char_patterns(uppercase, lowercase, symbol, digit);
    let char_pattern_with_length = random_char_pattern_length(char_patterns, length as Length, rng);

    let mut random_chars: Vec<char> = char_pattern_with_length.iter().flat_map(|(&pattern, &length)| {
        match pattern {
            CharPattern::Digit => Digit::random_choose_multiple(length as usize, rng),
            CharPattern::Lowercase => Lowercase::random_choose_multiple(length as usize, rng),
            CharPattern::Uppercase => Uppercase::random_choose_multiple(length as usize, rng),
            CharPattern::Symbol => Symbol::random_choose_multiple(length as usize, rng),
        }
    }).collect();

    random_chars.shuffle(rng);
    random_chars.shuffle(rng);
    random_chars.shuffle(rng);

    random_chars
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_random_vec() {
        let length = 10;
        let wants = 5;
        let mut rng = rand::thread_rng();
        let result = generate_random_vec(length, wants, &mut rng);

        assert_eq!(result.len(), wants as usize);
        assert_eq!(result.iter().sum::<usize>(), length);
    }

    #[test]
    fn test_random_char_pattern_length() {
        let patterns = vec![CharPattern::Uppercase, CharPattern::Lowercase, CharPattern::Symbol, CharPattern::Digit];
        let length = 10;
        let mut rng = rand::thread_rng();
        let result = random_char_pattern_length(patterns.clone(), length, &mut rng);

        assert_eq!(result.len(), patterns.len());
        assert_eq!(result.values().sum::<usize>(), length);
        for &pattern in &patterns {
            assert!(result.contains_key(&pattern));
        }
    }

    #[test]
    fn test_get_char_patterns() {
        let result = get_char_patterns(true, true, true, true);

        assert_eq!(result.len(), 4);
        let result_set: HashSet<_> = result.iter().collect();
        assert!(result_set.contains(&CharPattern::Uppercase));
        assert!(result_set.contains(&CharPattern::Lowercase));
        assert!(result_set.contains(&CharPattern::Symbol));
        assert!(result_set.contains(&CharPattern::Digit));
    }

    #[test]
    fn test_random_chars() {
        let length = 10;
        let uppercase = true;
        let lowercase = true;
        let symbol = true;
        let digit = true;
        let mut rng = rand::thread_rng();

        let result = random_chars(length, uppercase, lowercase, symbol, digit, &mut rng);

        assert_eq!(result.len(), length as usize);
    }
}
