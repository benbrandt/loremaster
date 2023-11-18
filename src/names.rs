/*!
Name generates for the different Heroic Cultures
*/
use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::{EnumIter, EnumString};

use self::{
    dwarves::DwarfOfDurinsFolk,
    elves::ElfOfLindon,
    hobbits::HobbitOfTheShire,
    men::{Barding, ManOfBree},
};

mod dwarves;
mod elves;
mod hobbits;
mod men;

/// Trait that ensures a given struct can randomly generate a name.
pub trait NameGenerator: fmt::Display + Sized
where
    Standard: Distribution<Self>,
{
}

/// Generate a name for one of the following Heroic Cultures
#[derive(Clone, Copy, Debug, strum::Display, EnumIter, EnumString, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Name {
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
}

impl Name {
    /// Generate a new name for the given race
    ///
    /// ```
    /// use rand::Rng;
    /// use loremaster::names::Name;
    ///
    /// let name = Name::Hobbit.gen(&mut rand::thread_rng());
    /// ```
    pub fn gen<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        match self {
            Name::Bardings => rng.gen::<Barding>().to_string(),
            Name::DwarvesOfDurinsFolk => rng.gen::<DwarfOfDurinsFolk>().to_string(),
            Name::ElvesOfLindon => rng.gen::<ElfOfLindon>().to_string(),
            Name::HobbitsOfTheShire => rng.gen::<HobbitOfTheShire>().to_string(),
            Name::MenOfBree => rng.gen::<ManOfBree>().to_string(),
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
        for culture in Name::iter() {
            let name = culture.gen(&mut rng);
            assert!(!name.to_string().is_empty());
        }
    }

    #[test]
    fn can_parse_from_strings() {
        for culture in Name::iter() {
            assert_eq!(Ok(culture), Name::try_from(culture.to_string().as_str()));
        }
    }

    #[test]
    fn returns_error_for_unknown_string() {
        assert_eq!(Err(ParseError::VariantNotFound), Name::try_from("foo"));
    }
}
