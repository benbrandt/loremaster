/*!
Name generates for the different Heroic Cultures
*/
use core::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::EnumIter;

use self::hobbit::Hobbit;

mod hobbit;

/// Trait that ensures a given struct can randomly generate a name.
pub trait NameGenerator: fmt::Display + Sized
where
    Standard: Distribution<Self>,
{
}

/// Generate a name for one of the following Heroic Cultures
#[derive(Debug, EnumIter)]
pub enum Name {
    /// Names are composed of a first name and a family name. First names for men are usually
    /// simple and short, with women being often given names of flowers or precious stones, but
    /// among the older families a custom survives of giving more heroic and high-sounding names,
    /// whose origin can be traced back to a time before the Shire.
    Hobbit,
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
            Name::Hobbit => rng.gen::<Hobbit>().to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

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
}
