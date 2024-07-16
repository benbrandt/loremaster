#![allow(unsafe_op_in_unsafe_fn)]

use bindings::{
    exports::loremaster::characters::generate::Guest,
    loremaster::{
        characters::types::{Character, HeroicCulture},
        cultures::generate::{generate_culture, generate_name},
    },
};

#[allow(warnings)]
mod bindings;

trait Generator {
    fn generate_culture() -> HeroicCulture;
    fn generate_name(culture: HeroicCulture) -> String;
}

impl Character {
    fn new(heroic_culture: HeroicCulture, name: String) -> Self {
        Self {
            heroic_culture,
            name,
        }
    }

    fn generate<G>() -> Self
    where
        G: Generator,
    {
        let heroic_culture = G::generate_culture();
        let name = G::generate_name(heroic_culture);
        Self::new(heroic_culture, name)
    }
}

struct Component;

impl Generator for Component {
    fn generate_culture() -> HeroicCulture {
        generate_culture()
    }

    fn generate_name(culture: HeroicCulture) -> String {
        generate_name(culture)
    }
}

impl Guest for Component {
    fn generate_character() -> Character {
        Character::generate::<Self>()
    }
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod test {
    use super::*;

    struct MockGenerator;

    impl Generator for MockGenerator {
        fn generate_culture() -> HeroicCulture {
            HeroicCulture::Bardings
        }

        fn generate_name(_: HeroicCulture) -> String {
            "Bard".to_string()
        }
    }

    #[test]
    fn character_generated_with_name() {
        let character = Character::generate::<MockGenerator>();
        assert_eq!(character.heroic_culture, MockGenerator::generate_culture());
        assert_eq!(
            character.name,
            MockGenerator::generate_name(character.heroic_culture)
        );
    }
}
