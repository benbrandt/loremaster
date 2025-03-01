use std::fmt;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
};

#[derive(Debug)]
pub struct DwarfOfDurinsFolkName {
    name: &'static str,
}

impl fmt::Display for DwarfOfDurinsFolkName {
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

impl Distribution<DwarfOfDurinsFolkName> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DwarfOfDurinsFolkName {
        DwarfOfDurinsFolkName {
            name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rand::rng_from_os_rng;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let mut rng = rng_from_os_rng();
        let name = rng.random::<DwarfOfDurinsFolkName>();

        assert_eq!(format!("{name}"), format!("{}", name.name));
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rng_from_os_rng();
        let name = rng.random::<DwarfOfDurinsFolkName>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}
