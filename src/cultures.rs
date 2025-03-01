/*!
# Heroic Cultures
*/
use rand::{
    distr::{Distribution, StandardUniform},
    seq::IteratorRandom,
    Rng,
};
use serde::Serialize;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use utoipa::ToSchema;

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

/// Generate a name for one of the following Heroic Cultures
#[derive(
    Clone, Copy, Debug, strum::Display, EnumIter, EnumString, Eq, PartialEq, Serialize, ToSchema,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum HeroicCulture {
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

impl HeroicCulture {
    /// Generate a new name for the given race
    ///
    /// ```
    /// use rand::Rng;
    /// use cultures::HeroicCulture;
    ///
    /// let name = HeroicCulture::HobbitsOfTheShire.random_name(&mut rand::thread_rng());
    /// ```
    pub fn random_name<R: Rng + ?Sized>(self, rng: &mut R) -> String {
        match self {
            HeroicCulture::Bardings => rng.random::<BardingName>().to_string(),
            HeroicCulture::DwarvesOfDurinsFolk => rng.random::<DwarfOfDurinsFolkName>().to_string(),
            HeroicCulture::ElvesOfLindon => rng.random::<ElfOfLindonName>().to_string(),
            HeroicCulture::HobbitsOfTheShire => rng.random::<HobbitOfTheShireName>().to_string(),
            HeroicCulture::MenOfBree => rng.random::<ManOfBreeName>().to_string(),
            HeroicCulture::RangersOfTheNorth => rng.random::<RangerOfTheNorthName>().to_string(),
        }
    }
}

impl Distribution<HeroicCulture> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HeroicCulture {
        HeroicCulture::iter().choose(rng).unwrap()
    }
}

#[cfg(test)]
mod test {
    use strum::{IntoEnumIterator, ParseError};

    use crate::rand::rng_from_os_rng;

    use super::*;

    #[test]
    fn no_generated_names_are_empty() {
        let mut rng = rng_from_os_rng();
        for culture in HeroicCulture::iter() {
            let name = culture.random_name(&mut rng);
            assert!(!name.to_string().is_empty());
        }
    }

    #[test]
    fn can_parse_from_strings() {
        for culture in HeroicCulture::iter() {
            assert_eq!(
                Ok(culture),
                HeroicCulture::try_from(culture.to_string().as_str())
            );
        }
    }

    #[test]
    fn returns_error_for_unknown_string() {
        assert_eq!(
            Err(ParseError::VariantNotFound),
            HeroicCulture::try_from("foo")
        );
    }
}
