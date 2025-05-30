use std::fmt;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
};

#[derive(Debug)]
pub struct BardingName {
    name: &'static str,
}

impl fmt::Display for BardingName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const MALE_NAMES: &[&str] = &[
    // Core Rules
    "Aegir", "Arn", "Brandulf", "Domarr", "Egil", "Erland", "Farald", "Finn", "Gautarr", "Hafgrim",
    "Hjalmar", "Ingolf", "Jofur", "Kolbeinn", "Leiknir", "Lomund", "Munan", "Nari", "Nefstan",
    "Ottarr", "Ragnarr", "Reinald", "Sigmarr", "Steinarr", "Thorald", "Torwald", "Ulfarr",
    "Unnarr", "Vandil", "Varinn",
];
const FEMALE_NAMES: &[&str] = &[
    // Core Rules
    "Aldis", "Asfrid", "Bera", "Bergdis", "Dagmar", "Eilif", "Erna", "Frida", "Geira", "Gudrun",
    "Halla", "Hild", "Ingirun", "Ingrith", "Lif", "Linhild", "Kelda", "Runa", "Saldis", "Sigga",
    "Sigrun", "Thora", "Thordis", "Thorhild", "Ulfhild", "Ulfrun", "Una", "Valdis", "Vigdis",
    "Walda",
];

impl Distribution<BardingName> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BardingName {
        BardingName {
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
        let name = rng.random::<BardingName>();

        assert_eq!(format!("{name}"), format!("{}", name.name));
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rng_from_os_rng();
        let name = rng.random::<BardingName>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}
