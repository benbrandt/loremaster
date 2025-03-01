use std::fmt;

use rand::{
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
    Rng,
};

#[derive(Debug)]
pub struct RangerOfTheNorthName {
    name: &'static str,
}

impl fmt::Display for RangerOfTheNorthName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const MALE_NAMES: &[&str] = &[
    // Core Rules
    "Adrahil",
    "Amlaith",
    "Arvegil",
    "Baranor",
    "Belecthor",
    "Bergil",
    "Celepharn",
    "Cirion",
    "Damrod",
    "Dírhael",
    "Duinhir",
    "Egalmoth",
    "Eradan",
    "Findemir",
    "Forlong",
    "Golasdan",
    "Hallas",
    "Hirluin",
    "Ingold",
    "Iorlas",
    "Malvegil",
    "Ohtar",
    "Orodreth",
    "Tarannon",
    "Targon",
];
const FEMALE_NAMES: &[&str] = &[
    // Core Rules
    "Anwen",
    "Arbereth",
    "Berúthiel",
    "Baraniel",
    "Calanril",
    "Celenneth",
    "Elnîth",
    "Eraniel",
    "Finduilas",
    "Gilraen",
    "Gilraeth",
    "Gloredhel",
    "Idril",
    "Ioreth",
    "Ivorwen",
    "Lôrwend",
    "Lothíriel",
    "Luindîs",
    "Meneloth",
    "Moriel",
    "Morwen",
    "Narieth",
    "Narniel",
    "Orothêl",
    "Tarandîs",
];

impl Distribution<RangerOfTheNorthName> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangerOfTheNorthName {
        RangerOfTheNorthName {
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
        let name = rng.random::<RangerOfTheNorthName>();

        assert_eq!(format!("{name}"), format!("{}", name.name));
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rng_from_os_rng();
        let name = rng.random::<RangerOfTheNorthName>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}
