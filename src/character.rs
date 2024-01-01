/*!
# Character information

Contains the information necessary to fill out a character sheet.
*/

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::Serialize;

use crate::cultures::HeroicCulture;

/// Contains the information necessary to fill out a character sheet.
#[derive(Debug, Serialize)]
pub struct Character {
    heroic_culture: HeroicCulture,
    name: String,
}

impl Character {
    /// Generate a new character based on a given heroic culture
    pub fn new<R: Rng + ?Sized>(rng: &mut R, heroic_culture: HeroicCulture) -> Self {
        Self {
            heroic_culture,
            name: heroic_culture.gen_name(rng),
        }
    }
}

impl Distribution<Character> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        let heroic_culture = rng.gen::<HeroicCulture>();
        Character::new(rng, heroic_culture)
    }
}

#[cfg(test)]
mod test {
    use crate::rand_utils;

    use super::*;

    #[test]
    fn character_generated_with_name() {
        let mut rng = rand_utils::rng_from_entropy();
        let character = rng.gen::<Character>();

        assert!(!character.name.is_empty());
    }
}
