use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use crate::names::NameGenerator;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ManOfBree {
    first_name: &'static str,
    family_name: &'static str,
}

impl fmt::Display for ManOfBree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.family_name)
    }
}

const MALE_NAMES: &[&str] = &[
    // Core Rules
    "Alfred", "Artie", "Bill", "Bob", "Carl", "Ed", "Fred", "Giles", "Herb", "Larry", "Nob",
    "Oswald", "Percy", "Perry", "Sid", "Tom", "Harry",
];
const FEMALE_NAMES: &[&str] = &[
    // Core Rules
    "Daisy", "Emma", "Etta", "Fay", "Fern", "Flora", "Gert", "Holly", "Lily", "Myrtle", "Poppy",
    "Rose", "Sage", "Tilly", "Violet",
];
const FAMILY_NAMES: &[&str] = &[
    // Core Rules
    "Appledore",
    "Asterfire",
    "Bellsap",
    "Briarcleave",
    "Butterbur",
    "Cherryborn",
    "Chesterstout",
    "Droverwind",
    "Ferny",
    "Foxglow",
    "Goatleaf",
    "Hardybough",
    "Heathertoes",
    "Hedgedon",
    "Kettlegrass",
    "Lilyhawk",
    "Mossburn",
    "Mugworts",
    "Oakstout",
    "Pickthorn",
    "Pollenroad",
    "Rushlight",
    "Shrubrose",
    "Sweetroot",
    "Thistlewool",
    "Wayward",
];

impl Distribution<ManOfBree> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ManOfBree {
        ManOfBree {
            first_name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
            family_name: FAMILY_NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for ManOfBree {}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rand_utils;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let name = ManOfBree {
            first_name: "First",
            family_name: "Family",
        };
        assert_eq!(format!("{name}"), "First Family");
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<ManOfBree>();

        assert!([MALE_NAMES, FEMALE_NAMES]
            .concat()
            .contains(&name.first_name));
        assert!(FAMILY_NAMES.contains(&name.family_name));
    }
}
