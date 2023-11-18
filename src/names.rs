/*!
Name generates for the different Heroic Cultures
*/
use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::{EnumIter, EnumString};

use self::{bardings::Barding, hobbits::HobbitOfTheShire};

mod bardings;
mod hobbits;

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
    /// Hobbits speak only the Common Speech, preserving the use of a few words and names of their
    /// own forgotten tongue. Names are composed of a first name and a family name. First names for
    /// men are usually simple and short, with women being often given names of flowers or precious
    /// stones, but among the older families a custom survives of giving more heroic and
    /// high-sounding names, whose origin can be traced back to a time before the Shire.
    HobbitsOfTheShire,
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
            Name::HobbitsOfTheShire => rng.gen::<HobbitOfTheShire>().to_string(),
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
