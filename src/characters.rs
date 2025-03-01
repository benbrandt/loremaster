/*!
# Character information

Contains the information necessary to fill out a character sheet.
*/

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
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

impl Distribution<Character> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        let heroic_culture = rng.random::<HeroicCulture>();
        Character::new(heroic_culture, heroic_culture.random_name(rng))
    }
}

#[cfg(test)]
mod test {
    use crate::rand::rng_from_os_rng;

    use super::*;

    #[test]
    fn character_generated_with_name() {
        let mut rng = rng_from_os_rng();
        let character = rng.random::<Character>();

        assert!(!character.name.is_empty());
    }
}
