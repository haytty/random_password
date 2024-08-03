use rand::prelude::SliceRandom;
use rand::Rng;

pub trait Random {
    fn available_values() -> &'static [char];
    fn random_choose_multiple<R>(length: usize, rng: &mut R) -> Vec<char>
    where
        R: Rng + ?Sized,
    {
        let chars = Self::available_values()
            .choose_multiple(rng, length)
            .cloned()
            .collect();
        chars
    }
}