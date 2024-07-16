#![allow(unsafe_op_in_unsafe_fn)]

use bardings::BardingName;
use bindings::{
    exports::loremaster::cultures::generate::Guest, loremaster::cultures::types::HeroicCulture,
};
use bree::ManOfBreeName;
use dwarves::DwarfOfDurinsFolkName;
use elves::ElfOfLindonName;
use hobbits::HobbitOfTheShireName;
use rand::{
    distributions::{Distribution, Standard},
    seq::IteratorRandom,
    Rng,
};
use rangers::RangerOfTheNorthName;
use strum::IntoEnumIterator;

mod bardings;
#[allow(warnings)]
mod bindings;
mod bree;
mod dwarves;
mod elves;
mod hobbits;
mod rangers;

struct Component;

impl Guest for Component {
    fn generate_culture() -> HeroicCulture {
        rand_utils::rng_from_entropy().r#gen::<HeroicCulture>()
    }

    fn generate_name(culture: HeroicCulture) -> String {
        culture.generate_name(&mut rand_utils::rng_from_entropy())
    }
}

impl HeroicCulture {
    /// Generate a random name for a given Heroic Culture.
    pub fn generate_name<R: Rng + ?Sized>(self, rng: &mut R) -> String {
        match self {
            HeroicCulture::Bardings => rng.r#gen::<BardingName>().to_string(),
            HeroicCulture::DwarvesOfDurinsFolk => rng.r#gen::<DwarfOfDurinsFolkName>().to_string(),
            HeroicCulture::ElvesOfLindon => rng.r#gen::<ElfOfLindonName>().to_string(),
            HeroicCulture::HobbitsOfTheShire => rng.r#gen::<HobbitOfTheShireName>().to_string(),
            HeroicCulture::MenOfBree => rng.r#gen::<ManOfBreeName>().to_string(),
            HeroicCulture::RangersOfTheNorth => rng.r#gen::<RangerOfTheNorthName>().to_string(),
        }
    }
}

impl Distribution<HeroicCulture> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HeroicCulture {
        HeroicCulture::iter().choose(rng).unwrap()
    }
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn random_culture() {
        let mut rng = rand_utils::rng_from_entropy();
        rng.r#gen::<HeroicCulture>();
    }

    #[test]
    fn binding() {
        let culture = Component::generate_culture();
        Component::generate_name(culture);
    }

    #[test]
    fn no_generated_names_are_empty() {
        let mut rng = rand_utils::rng_from_entropy();
        for culture in HeroicCulture::iter() {
            let name = culture.generate_name(&mut rng);
            assert!(!name.to_string().is_empty());
        }
    }

    // #[test]
    // fn can_parse_from_strings() {
    //     for culture in HeroicCulture::iter() {
    //         assert_eq!(
    //             Ok(culture),
    //             HeroicCulture::try_from(culture.to_string().as_str())
    //         );
    //     }
    // }

    // #[test]
    // fn returns_error_for_unknown_string() {
    //     assert_eq!(
    //         Err(ParseError::VariantNotFound),
    //         HeroicCulture::try_from("foo")
    //     );
    // }
}
