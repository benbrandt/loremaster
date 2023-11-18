use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use crate::names::NameGenerator;

#[derive(Debug)]
pub struct RangerOfTheNorth {
    name: &'static str,
}

impl fmt::Display for RangerOfTheNorth {
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

impl Distribution<RangerOfTheNorth> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangerOfTheNorth {
        RangerOfTheNorth {
            name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

impl NameGenerator for RangerOfTheNorth {}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rand_utils;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<RangerOfTheNorth>();

        assert_eq!(format!("{name}"), format!("{}", name.name));
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<RangerOfTheNorth>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}
