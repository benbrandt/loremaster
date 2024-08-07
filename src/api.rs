use std::{fmt, str::FromStr};

use serde::Serialize;
use spin_sdk::{
    http::{conversions::TryIntoBody, Json, Params, Request, Response},
    http_router,
};
use utoipa::{OpenApi, ToSchema};
use utoipa_scalar::Scalar;

use crate::bindings::loremaster::characters::types::{Character, HeroicCulture};

pub trait Generator {
    fn generate_character() -> Character;
    fn generate_name(culture: HeroicCulture) -> String;
}

#[derive(OpenApi)]
#[openapi(
    paths(openapi, characters, names),
    components(schemas(CharacterRef, HeroicCultureRef))
)]
struct ApiDoc;

// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
fn openapi(_: Request, _: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Json(ApiDoc::openapi()).try_into_body()?)
        .build())
}

fn scalar(_: Request, _: Params) -> Response {
    let scalar = Scalar::new(ApiDoc::openapi());
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(scalar.to_html())
        .build()
}

pub fn router<G: Generator + 'static>(req: Request) -> Response {
    if let Some(header) = req.header("spin-full-url") {
        println!(
            "Handling request to {}",
            header.as_str().unwrap_or_default()
        );
    }

    let router = http_router! {
        POST "/characters" => characters::<G>,
        POST "/cultures/:culture/names" => names::<G>,
        GET  "/api-docs" => scalar,
        GET  "/api-docs/openapi.json" => openapi,
        _   "/*"             => |_req: Request, _| {
            Response::new(404, "")
        }
    };
    router.handle(req)
}

// POST /characters
#[utoipa::path(
    post,
    path = "/characters",
    responses(
        (status = 200, description = "Character", body = Character)
    )
)]
fn characters<G: Generator>(_req: Request, _params: Params) -> anyhow::Result<Response> {
    let character = G::generate_character();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Json(CharacterJson(character)).try_into_body()?)
        .build())
}

// POST /cultures/:culture/names
#[utoipa::path(
    post,
    path = "/cultures/{culture}/names",
    responses(
        (status = 200, description = "Name", body = String)
    ),
    params(
        ("culture" = HeroicCulture, Path, description = "Heroic Culture to generate a name from"),
    )
)]
fn names<G: Generator>(_req: Request, params: Params) -> anyhow::Result<Response> {
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

#[derive(Serialize, ToSchema)]
#[schema(as = HeroicCulture)]
#[serde(remote = "HeroicCulture", rename_all = "kebab-case")]
enum HeroicCultureRef {
    Bardings,
    DwarvesOfDurinsFolk,
    ElvesOfLindon,
    HobbitsOfTheShire,
    MenOfBree,
    RangersOfTheNorth,
}

#[derive(Serialize, ToSchema)]
#[schema(as = Character)]
#[serde(remote = "Character")]
struct CharacterRef {
    #[serde(with = "HeroicCultureRef")]
    heroic_culture: HeroicCulture,
    name: String,
}

#[derive(Serialize)]
struct CharacterJson(#[serde(with = "CharacterRef")] Character);

#[cfg(test)]
mod test {
    use routefinder::Capture;
    use serde_json::Value;
    use spin_sdk::http::Method;

    use crate::bindings::loremaster::cultures::types::HeroicCulture;

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

    #[derive(Default)]
    struct CultureIterator(Option<HeroicCulture>);

    impl CultureIterator {
        #[must_use]
        fn new() -> Self {
            Self::default()
        }
    }

    impl Iterator for CultureIterator {
        type Item = HeroicCulture;

        fn next(&mut self) -> Option<Self::Item> {
            self.0 = match self.0 {
                None => Some(HeroicCulture::Bardings),
                Some(HeroicCulture::Bardings) => Some(HeroicCulture::DwarvesOfDurinsFolk),
                Some(HeroicCulture::DwarvesOfDurinsFolk) => Some(HeroicCulture::ElvesOfLindon),
                Some(HeroicCulture::ElvesOfLindon) => Some(HeroicCulture::HobbitsOfTheShire),
                Some(HeroicCulture::HobbitsOfTheShire) => Some(HeroicCulture::MenOfBree),
                Some(HeroicCulture::MenOfBree) => Some(HeroicCulture::RangersOfTheNorth),
                Some(HeroicCulture::RangersOfTheNorth) => None,
            };
            self.0
        }
    }

    #[test]
    fn openapi_docs() {
        for uri in ["/api-docs", "/api-docs/openapi.json"] {
            let response = router::<MockGenerator>(Request::new(Method::Get, uri));
            let body = response.body();
            assert_eq!(response.status(), &200);
            assert!(!body.is_empty());
        }
    }

    #[test]
    fn returns_a_character() {
        let response = router::<MockGenerator>(Request::new(Method::Post, "/characters"));
        let body = serde_json::from_slice::<Value>(response.body()).unwrap();

        assert_eq!(body["heroic_culture"].as_str().unwrap(), "bardings");
        assert_eq!(body["name"].as_str().unwrap(), "Bard");
    }

    #[test]
    fn returns_a_name() {
        for culture in CultureIterator::new() {
            let response = router::<MockGenerator>(Request::new(
                Method::Post,
                format!("/cultures/{culture}/names"),
            ));
            assert_eq!(response.status(), &200u16);
        }
    }

    #[test]
    fn unknown_route() {
        let request = Request::get("/unknown")
            .header("spin-full-url", "http://localhost:8080/unknown")
            .build();
        let response = router::<MockGenerator>(request);

        assert_eq!(response.status(), &404);
    }

    #[test]
    fn character_route_returns_a_character() {
        let response =
            characters::<MockGenerator>(Request::new(Method::Post, "/characters"), Params::new())
                .unwrap();
        let body = serde_json::from_slice::<Value>(response.body()).unwrap();

        assert!(!body["heroic_culture"].as_str().unwrap().is_empty());
        assert!(!body["name"].as_str().unwrap().is_empty());
    }

    #[test]
    fn name_route_returns_a_name() {
        for culture in CultureIterator::new() {
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
        for culture in CultureIterator::new() {
            assert_eq!(culture, culture.to_string().as_str().parse().unwrap());
        }
    }

    #[test]
    fn returns_error_for_unknown_string() {
        assert!("foo".parse::<HeroicCulture>().is_err());
    }
}
