use rand::Rng;
use spin_sdk::{
    http::{conversions::TryIntoBody, Json, Params, Request, Response},
    http_router,
};
use utoipa::OpenApi;
use utoipa_scalar::Scalar;

use crate::{characters::Character, cultures::HeroicCulture, rand::rng_from_os_rng};

#[derive(OpenApi)]
#[openapi(
    paths(openapi, characters, names),
    components(schemas(Character, HeroicCulture))
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

pub fn router(req: Request) -> Response {
    if let Some(header) = req.header("spin-full-url") {
        println!(
            "Handling request to {}",
            header.as_str().unwrap_or_default()
        );
    }

    let router = http_router! {
        POST "/characters" => characters,
        POST "/cultures/:culture/names" => names,
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
fn characters(_req: Request, _params: Params) -> anyhow::Result<Response> {
    let character = rng_from_os_rng().random::<Character>();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Json(character).try_into_body()?)
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
fn names(_req: Request, params: Params) -> anyhow::Result<Response> {
    let culture = params
        .get("culture")
        .expect("CULTURE param missing")
        .parse::<HeroicCulture>()?;

    let name = culture.random_name(&mut rng_from_os_rng());

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

    use super::*;

    #[test]
    fn openapi_docs() {
        for uri in ["/api-docs", "/api-docs/openapi.json"] {
            let response = router(Request::new(Method::Get, uri));
            let body = response.body();
            assert_eq!(response.status(), &200);
            assert!(!body.is_empty());
        }
    }

    #[test]
    fn returns_a_character() {
        let response = router(Request::new(Method::Post, "/characters"));
        let body = serde_json::from_slice::<Value>(response.body()).unwrap();

        assert!(!body["heroic_culture"].as_str().unwrap().is_empty());
        assert!(!body["name"].as_str().unwrap().is_empty());
    }

    #[test]
    fn returns_a_name() {
        for culture in HeroicCulture::iter() {
            let response = router(Request::new(
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
        let response = router(request);

        assert_eq!(response.status(), &404);
    }

    #[test]
    fn character_route_returns_a_character() {
        let response =
            characters(Request::new(Method::Post, "/characters"), Params::new()).unwrap();
        let body = serde_json::from_slice::<Value>(response.body()).unwrap();

        assert!(!body["heroic_culture"].as_str().unwrap().is_empty());
        assert!(!body["name"].as_str().unwrap().is_empty());
    }

    #[test]
    fn name_route_returns_a_name() {
        for culture in HeroicCulture::iter() {
            let response = names(
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
