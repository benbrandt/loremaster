/*!
# Character information

Contains the information necessary to fill out a character sheet.
*/

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::cultures::HeroicCulture;

/// Contains the information necessary to fill out a character sheet.
#[derive(Debug, Serialize, ToSchema)]
pub struct Character {
    heroic_culture: HeroicCulture,
    name: String,
}

impl Character {
    #[must_use]
    pub fn new(heroic_culture: HeroicCulture, name: impl Into<String>) -> Self {
        Self {
            heroic_culture,
            name: name.into(),
        }
    }
}

impl Distribution<Character> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        let heroic_culture = rng.r#gen::<HeroicCulture>();
        Character::new(heroic_culture, heroic_culture.generate_name(rng))
    }
}

#[cfg(test)]
mod test {
    use crate::rand::rng_from_entropy;

    use super::*;

    #[test]
    fn character_generated_with_name() {
        let mut rng = rng_from_entropy();
        let character = rng.r#gen::<Character>();

        assert!(!character.name.is_empty());
    }
}
