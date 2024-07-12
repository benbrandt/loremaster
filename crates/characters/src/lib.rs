/*!
# Character information

Contains the information necessary to fill out a character sheet.
*/

use cultures::HeroicCulture;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::Serialize;

/// Contains the information necessary to fill out a character sheet.
#[derive(Debug, Serialize)]
pub struct Character {
    heroic_culture: HeroicCulture,
    name: String,
}

impl Character {
    #[must_use]
    pub fn new(heroic_culture: HeroicCulture, name: String) -> Self {
        Self {
            heroic_culture,
            name,
        }
    }
}

impl Distribution<Character> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        let heroic_culture = rng.r#gen::<HeroicCulture>();
        Character::new(heroic_culture, heroic_culture.gen_name(rng))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn character_generated_with_name() {
        let mut rng = rand_utils::rng_from_entropy();
        let character = rng.r#gen::<Character>();

        assert!(!character.name.is_empty());
    }
}
