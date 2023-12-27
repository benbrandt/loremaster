use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use super::NameGenerator;

#[derive(Debug)]
pub struct DwarfOfDurinsFolk {
    name: &'static str,
}

impl fmt::Display for DwarfOfDurinsFolk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const MALE_NAMES: &[&str] = &[
    // Core Rules
    "Ai", "Anar", "Beli", "Bláin", "Borin", "Burin", "Bruni", "Farin", "Flói", "Frár", "Frerin",
    "Frór", "Ginar", "Gróin", "Grór", "Hanar", "Hepti", "Iari", "Lófar", "Lóni", "Náli", "Nár",
    "Niping", "Nói", "Núr", "Nýrád", "Ónar", "Póri", "Regin", "Svior", "Veig", "Vida",
];
const FEMALE_NAMES: &[&str] = &[
    // Core Rules
    "Adís", "Afrid", "Agda", "Bersa", "Birna", "Dagrún", "Dís", "Drífa", "Edda", "Elin", "Fenja",
    "Frida", "Geira", "Gísla", "Hadda", "Hón", "Ida", "Ilmr", "Jóra", "Kára", "Kóna", "Líf",
    "Línhild", "Már", "Mist", "Nál", "Oda", "Ósk", "Rán", "Rinda", "Sefa", "Syn", "Tóra", "Trana",
    "Úlfrún", "Vírún", "Yrr",
];

impl Distribution<DwarfOfDurinsFolk> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DwarfOfDurinsFolk {
        DwarfOfDurinsFolk {
            name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

impl NameGenerator for DwarfOfDurinsFolk {}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rand_utils;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<DwarfOfDurinsFolk>();

        assert_eq!(format!("{name}"), format!("{}", name.name));
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<DwarfOfDurinsFolk>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}