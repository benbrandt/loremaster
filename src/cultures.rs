/*!
# Heroic Cultures
*/
use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::{EnumIter, EnumString};

use self::{
    bardings::BardingName, bree::ManOfBreeName, dwarves::DwarfOfDurinsFolkName,
    elves::ElfOfLindonName, hobbits::HobbitOfTheShireName, rangers::RangerOfTheNorthName,
};

mod bardings;
mod bree;
mod dwarves;
mod elves;
mod hobbits;
mod rangers;

/// Trait that ensures a given struct can randomly generate a name.
pub trait NameGenerator: fmt::Display + Sized
where
    Standard: Distribution<Self>,
{
}

/// Generate a name for one of the following Heroic Cultures
#[derive(Clone, Copy, Debug, strum::Display, EnumIter, EnumString, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Culture {
    /// All Bardings speak Dalish, a language that can be described as a very old form of the
    /// Common Speech. As far as their names are concerned, they are usually composed of one or two
    /// elements (for example, Dag — Day, or Lif-stan — Life Stone). Like most Northmen, Bardings
    /// often name their children after a renowned ancestor or relative, or choose a name beginning
    /// with the same sound or sharing one element with that of the father (whose name is often
    /// given with their first name when introduced formally — for example, Lifstan, son of
    /// Leiknir, or Ingrith, daughter of Ingolf).
    Bardings,
    /// All Dwarves speak the Common Tongue, but preserve a knowledge of a secret Dwarvish
    /// language. They receive a true name at birth that they do not reveal to members of other
    /// folks, and adopt another name in the tradition of their neighbours. This custom has been in
    /// use for so long that a number of names have become traditionally associated with Dwarves,
    /// and are used almost exclusively by them. Dwarves of renown are sometimes given an honorific
    /// title, celebrating an exceptional deed or distinctive quality (for example, Thorin
    /// Oakenshield or Dáin Ironfoot).
    DwarvesOfDurinsFolk,
    /// In addition to the Common Speech, all Elves speak their own, fair tongue — the Sindarin
    /// speech. For the most part, the Elves of Lindon bear names fashioned in that language.
    ElvesOfLindon,
    /// Hobbits speak only the Common Speech, preserving the use of a few words and names of their
    /// own forgotten tongue. Names are composed of a first name and a family name. First names for
    /// men are usually simple and short, with women being often given names of flowers or precious
    /// stones, but among the older families a custom survives of giving more heroic and
    /// high-sounding names, whose origin can be traced back to a time before the Shire.
    HobbitsOfTheShire,
    /// The Men of Bree have forgotten their ancient, native speech, and speak the Common Tongue,
    /// albeit slightly altered in a local dialect. They use names that to foreign ears sound
    /// similar to those used by Hobbits in the Shire (Hobbits beg to differ, of course).
    MenOfBree,
    /// The native language of the Dúnedain is the Westron, or Common Speech. Some still learn the
    /// Sindarin Elven-tongue, as it is handed down from generation to generation. They retain an
    /// ancient tradition of naming their children using that fair speech.
    RangersOfTheNorth,
}

impl Culture {
    /// Generate a new name for the given race
    ///
    /// ```
    /// use rand::Rng;
    /// use loremaster::names::Name;
    ///
    /// let name = Name::Hobbit.gen(&mut rand::thread_rng());
    /// ```
    pub fn gen_name<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        match self {
            Culture::Bardings => rng.gen::<BardingName>().to_string(),
            Culture::DwarvesOfDurinsFolk => rng.gen::<DwarfOfDurinsFolkName>().to_string(),
            Culture::ElvesOfLindon => rng.gen::<ElfOfLindonName>().to_string(),
            Culture::HobbitsOfTheShire => rng.gen::<HobbitOfTheShireName>().to_string(),
            Culture::MenOfBree => rng.gen::<ManOfBreeName>().to_string(),
            Culture::RangersOfTheNorth => rng.gen::<RangerOfTheNorthName>().to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use strum::{IntoEnumIterator, ParseError};

    use crate::rand_utils;

    use super::*;

    #[test]
    fn no_generated_names_are_empty() {
        let mut rng = rand_utils::rng_from_entropy();
        for culture in Culture::iter() {
            let name = culture.gen_name(&mut rng);
            assert!(!name.to_string().is_empty());
        }
    }

    #[test]
    fn can_parse_from_strings() {
        for culture in Culture::iter() {
            assert_eq!(Ok(culture), Culture::try_from(culture.to_string().as_str()));
        }
    }

    #[test]
    fn returns_error_for_unknown_string() {
        assert_eq!(Err(ParseError::VariantNotFound), Culture::try_from("foo"));
    }
}
