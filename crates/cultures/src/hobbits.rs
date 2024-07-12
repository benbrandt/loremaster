use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use super::Name;

#[derive(Debug)]
pub struct HobbitOfTheShireName {
    first_name: &'static str,
    family_name: &'static str,
}

impl fmt::Display for HobbitOfTheShireName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.family_name)
    }
}

const MALE_NAMES: &[&str] = &[
    // Core Rules
    "Andwise",
    "Berilac",
    "Bungo",
    "Cottar",
    "Doderic",
    "Dudo",
    "Erling",
    "Fastred",
    "Ferumbras",
    "Folco",
    "Gorhendad",
    "Griffo",
    "Halfred",
    "Hamson",
    "Ilberic",
    "Isembold",
    "Isengar",
    "Longo",
    "Marmadas",
    "Marroc",
    "Mungo",
    "Odo",
    "Orgulas",
    "Otho",
    "Posco",
    "Reginard",
    "Robin",
    "Rudigar",
    "Sadoc",
    "Saradas",
    "Tobold",
    "Tolman",
];
const FEMALE_NAMES: &[&str] = &[
    // Core Rules
    "Adaldrida",
    "Amaranth",
    "Asphodel",
    "Belba",
    "Bell",
    "Berylla",
    "Camellia",
    "Daisy",
    "Eglantine",
    "Estella",
    "Gilly",
    "Hanna",
    "Lily",
    "Malva",
    "Marigold",
    "May",
    "Melilot",
    "Menegilda",
    "Mentha",
    "Mirabella",
    "Myrtle",
    "Pearl",
    "Peony",
    "Pervinca",
    "Pimpernel",
    "Primrose",
    "Primula",
    "Prisca",
    "Rosamunda",
    "Ruby",
    "Salvia",
];
const FAMILY_NAMES: &[&str] = &[
    // Core Rules
    "Baggins",
    "Boffin",
    "Bolger",
    "Bracegirdle",
    "Brandybuck",
    "Brown",
    "Brownlock",
    "Bunce",
    "Burrows",
    "Cotton",
    "Gamgee",
    "Gardner",
    "Goldworthy",
    "Goodbody",
    "Goodchild",
    "Grubb",
    "Headstrong",
    "Hornblower",
    "Maggot",
    "Noakes",
    "North-tooks",
    "Proudfoot",
    "Puddifoot",
    "Roper",
    "Rumble",
    "Sackville",
    "Smallburrow",
    "Took",
    "Twofoot",
    "Whitfoot",
];

impl Distribution<HobbitOfTheShireName> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HobbitOfTheShireName {
        HobbitOfTheShireName {
            first_name: [MALE_NAMES, FEMALE_NAMES]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
            family_name: FAMILY_NAMES.choose(rng).unwrap(),
        }
    }
}

impl Name for HobbitOfTheShireName {}

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn name_can_be_displayed() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.r#gen::<HobbitOfTheShireName>();

        assert_eq!(
            format!("{name}"),
            format!("{} {}", name.first_name, name.family_name)
        );
    }

    #[test]
    fn name_can_be_randomly_generated() {
        let mut rng = rand_utils::rng_from_entropy();
        let name = rng.r#gen::<HobbitOfTheShireName>();

        assert!([MALE_NAMES, FEMALE_NAMES]
            .concat()
            .contains(&name.first_name));
        assert!(FAMILY_NAMES.contains(&name.family_name));
    }
}
