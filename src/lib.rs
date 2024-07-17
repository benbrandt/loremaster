/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
#![allow(clippy::needless_pass_by_value)]

use bindings::loremaster::{
    characters::{
        generate::generate_character,
        types::{Character, HeroicCulture},
    },
    cultures::generate::generate_name,
};
use spin_sdk::{
    http::{Request, Response},
    http_component, http_router,
};

#[allow(warnings)]
mod bindings;

pub trait Generator {
    fn generate_character() -> Character;
    fn generate_name(culture: HeroicCulture) -> String;
}

struct Component;

impl Generator for Component {
    fn generate_character() -> Character {
        generate_character()
    }

    fn generate_name(culture: HeroicCulture) -> String {
        generate_name(culture)
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_loremaster(req: Request) -> Response {
    if let Some(header) = req.header("spin-full-url") {
        println!(
            "Handling request to {}",
            header.as_str().unwrap_or_default()
        );
    }

    let router = http_router! {
        POST "/characters" => api::characters::<Component>,
        POST "/cultures/:culture/names" => api::names::<Component>,
        _   "/*"             => |_req: Request, params| {
            let capture = params.wildcard().unwrap_or_default();
            Response::new(200, capture.to_string())
        }
    };
    router.handle(req)
}

mod api {
    use std::{fmt, str::FromStr};

    use spin_sdk::http::{conversions::TryIntoBody, Json, Params, Request, Response};

    use crate::{bindings::loremaster::characters::types::HeroicCulture, Generator};

    impl fmt::Display for HeroicCulture {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Bardings => "bardings",
                    Self::DwarvesOfDurinsFolk => "dwarves-of-durins-folk",
                    Self::ElvesOfLindon => "elves-of-lindon",
                    Self::HobbitsOfTheShire => "hobbits-of-the-shire",
                    Self::MenOfBree => "men-of-bree",
                    Self::RangersOfTheNorth => "rangers-of-the-north",
                }
            )
        }
    }

    impl FromStr for HeroicCulture {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> anyhow::Result<Self> {
            match s {
                "bardings" => Ok(Self::Bardings),
                "dwarves-of-durins-folk" => Ok(Self::DwarvesOfDurinsFolk),
                "elves-of-lindon" => Ok(Self::ElvesOfLindon),
                "hobbits-of-the-shire" => Ok(Self::HobbitsOfTheShire),
                "men-of-bree" => Ok(Self::MenOfBree),
                "rangers-of-the-north" => Ok(Self::RangersOfTheNorth),
                _ => Err(anyhow::anyhow!("Invalid culture: {}", s)),
            }
        }
    }

    // POST /characters
    pub fn characters<G>(_req: Request, _params: Params) -> anyhow::Result<Response>
    where
        G: Generator,
    {
        let character = G::generate_character();

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(Json(character).try_into_body()?)
            .build())
    }

    // POST /cultures/:culture/names
    pub fn names<G>(_req: Request, params: Params) -> anyhow::Result<Response>
    where
        G: Generator,
    {
        let culture = params
            .get("culture")
            .expect("CULTURE param missing")
            .parse::<HeroicCulture>()?;

        let name = G::generate_name(culture);

        Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(name)
            .build())
    }

    #[cfg(test)]
    mod test {
        use routefinder::Capture;
        use serde_json::Value;
        use spin_sdk::http::Method;
        use strum::IntoEnumIterator;

        use crate::bindings::loremaster::{
            characters::types::Character, cultures::types::HeroicCulture,
        };

        use super::*;

        struct MockGenerator;

        impl Generator for MockGenerator {
            fn generate_character() -> Character {
                Character {
                    heroic_culture: HeroicCulture::Bardings,
                    name: "Bard".to_string(),
                }
            }

            fn generate_name(_culture: HeroicCulture) -> String {
                "Bard".to_string()
            }
        }

        #[test]
        fn returns_a_character() {
            let response = characters::<MockGenerator>(
                Request::new(Method::Post, "/characters"),
                Params::new(),
            )
            .unwrap();
            let body = serde_json::from_slice::<Value>(response.body()).unwrap();

            assert!(!body["heroic_culture"].as_str().unwrap().is_empty());
            assert!(!body["name"].as_str().unwrap().is_empty());
        }

        #[test]
        fn returns_a_name() {
            for culture in HeroicCulture::iter() {
                let response = names::<MockGenerator>(
                    Request::new(Method::Post, format!("/cultures/{culture}/names")),
                    Params::from_iter([Capture::new("culture", culture.to_string())]),
                )
                .unwrap();
                assert_eq!(response.status(), &200u16);
            }
        }

        #[test]
        fn can_parse_from_strings() {
            for culture in HeroicCulture::iter() {
                assert_eq!(culture, culture.to_string().as_str().parse().unwrap());
            }
        }

        #[test]
        fn returns_error_for_unknown_string() {
            assert!("foo".parse::<HeroicCulture>().is_err());
        }
    }
}

// #[cfg(test)]
// mod test {
//     use serde_json::Value;
//     use spin_sdk::http::Method;
//     use strum::IntoEnumIterator;

//     use super::*;

//     #[test]
//     fn returns_a_character() {
//         let response = handle_loremaster(Request::new(Method::Post, "/characters"));
//         let body = serde_json::from_slice::<Value>(response.body()).unwrap();

//         assert!(!body["heroic_culture"].as_str().unwrap().is_empty());
//         assert!(!body["name"].as_str().unwrap().is_empty());
//     }

//     #[test]
//     fn returns_a_name() {
//         for culture in HeroicCulture::iter() {
//             let response = handle_loremaster(Request::new(
//                 Method::Post,
//                 format!("/cultures/{culture}/names"),
//             ));
//             assert_eq!(response.status(), &200u16);
//         }
//     }
// }
