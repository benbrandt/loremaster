use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use super::NameGenerator;

#[derive(Debug)]
pub struct ElfOfLindonName {
    name: &'static str,
}

impl fmt::Display for ElfOfLindonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const MALE_NAMES: &[&str] = &[
    // Core Rules
    "Amras",
    "Aredhel",
    "Beleganor",
    "Belegon",
    "Calanhir",
    "Carmagor",
    "Dagorhir",
    "Durandir",
    "Edrahil",
    "Ellahir",
    "Fincalan",
    "Fuindor",
    "Galdagor",
    "Galdor",
    "Hallas",
    "Hirimlad",
    "Ithildir",
    "Lascalan",
    "Linaith",
    "Mablin",
    "Malanor",
    "Nauros",
    "Orgalad",
    "Pelegorn",
    "Sargon",
];
const FEMALE_NAMES: &[&str] = &[
    // Core Rules
    "Anórel",
    "Aranel",
    "Arbereth",
    "Baraniel",
    "Calanril",
    "Celebrindal",
    "Celenneth",
    "Elanor",
    "Elwing",
    "Eraniel",
    "Fimbrethil",
    "Gloredhel",
    "Idril",
    "Irilde",
    "Laurelin",
    "Lôrwend",
    "Lothíriel",
    "Meneloth",
    "Moriel",
    "Narieth",
    "Narniel",
    "Nimloth",
    "Nimrodel",
    "Níniel",
    "Tarandîs",
];

impl Distribution<ElfOfLindonName> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ElfOfLindonName {
        ElfOfLindonName {
            name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

impl NameGenerator for ElfOfLindonName {}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rand_utils;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<ElfOfLindonName>();

        assert_eq!(format!("{name}"), format!("{}", name.name));
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<ElfOfLindonName>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}
