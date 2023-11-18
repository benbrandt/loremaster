use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use super::NameGenerator;

#[derive(Debug)]
pub struct Barding {
    name: &'static str,
}

impl fmt::Display for Barding {
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

impl Distribution<Barding> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Barding {
        Barding {
            name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

impl NameGenerator for Barding {}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rand_utils;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let name = Barding { name: "First" };
        assert_eq!(format!("{name}"), "First");
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.gen::<Barding>();

        assert!([MALE_NAMES, FEMALE_NAMES].concat().contains(&name.name));
    }
}
