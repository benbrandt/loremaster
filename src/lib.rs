/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
#![allow(clippy::needless_pass_by_value)]

use spin_sdk::http::{Request, Response};
use spin_sdk::{http_component, http_router};

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
        POST "/characters" => api::characters,
        POST "/cultures/:culture/names" => api::names,
        _   "/*"             => |_req: Request, params| {
            let capture = params.wildcard().unwrap_or_default();
            Response::new(200, capture.to_string())
        }
    };
    router.handle(req)
}

mod api {
    use characters::Character;
    use cultures::HeroicCulture;
    use rand::Rng;
    use spin_sdk::http::{conversions::TryIntoBody, Json, Params, Request, Response};

    // POST /characters
    pub fn characters(_req: Request, _params: Params) -> anyhow::Result<Response> {
        let mut rng = rand_utils::rng_from_entropy();
        let character = rng.r#gen::<Character>();

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(Json(character).try_into_body()?)
            .build())
    }

    // POST /cultures/:culture/names
    pub fn names(_req: Request, params: Params) -> anyhow::Result<Response> {
        let culture = HeroicCulture::try_from(params.get("culture").expect("CULTURE"))?;
        let mut rng = rand_utils::rng_from_entropy();
        let name = culture.gen_name(&mut rng).to_string();

        Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(name)
            .build())
    }
}

#[cfg(test)]
mod test {
    use cultures::HeroicCulture;
    use serde_json::Value;
    use spin_sdk::http::Method;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn returns_a_character() {
        let response = handle_loremaster(Request::new(Method::Post, "/characters"));
        let body = serde_json::from_slice::<Value>(response.body()).unwrap();

        assert!(!body["heroic_culture"].as_str().unwrap().is_empty());
        assert!(!body["name"].as_str().unwrap().is_empty());
    }

    #[test]
    fn returns_a_name() {
        for culture in HeroicCulture::iter() {
            let response = handle_loremaster(Request::new(
                Method::Post,
                format!("/cultures/{culture}/names"),
            ));
            assert_eq!(response.status(), &200u16);
        }
    }
}
