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
    /// Iterate through all possible cultures
    fn iter() -> impl Iterator<Item = Self> {
        CultureIterator::new()
    }

    /// Generate a random name for a given Heroic Culture.
    fn generate_name<R: Rng + ?Sized>(self, rng: &mut R) -> String {
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

#[derive(Default)]
struct CultureIterator(Option<HeroicCulture>);

impl CultureIterator {
    fn new() -> Self {
        Self::default()
    }
}

impl Iterator for CultureIterator {
    type Item = HeroicCulture;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(HeroicCulture::Bardings),
            Some(HeroicCulture::Bardings) => Some(HeroicCulture::DwarvesOfDurinsFolk),
            Some(HeroicCulture::DwarvesOfDurinsFolk) => Some(HeroicCulture::ElvesOfLindon),
            Some(HeroicCulture::ElvesOfLindon) => Some(HeroicCulture::HobbitsOfTheShire),
            Some(HeroicCulture::HobbitsOfTheShire) => Some(HeroicCulture::MenOfBree),
            Some(HeroicCulture::MenOfBree) => Some(HeroicCulture::RangersOfTheNorth),
            Some(HeroicCulture::RangersOfTheNorth) => None,
        };
        self.0
    }
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterate_cultures() {
        let expected = &[
            HeroicCulture::Bardings,
            HeroicCulture::DwarvesOfDurinsFolk,
            HeroicCulture::ElvesOfLindon,
            HeroicCulture::HobbitsOfTheShire,
            HeroicCulture::MenOfBree,
            HeroicCulture::RangersOfTheNorth,
        ];

        assert_eq!(
            HeroicCulture::iter().collect::<Vec<_>>().as_slice(),
            expected
        );
    }

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
}
