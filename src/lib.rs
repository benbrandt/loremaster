/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
#![allow(clippy::needless_pass_by_value)]

use spin_sdk::http::{Request, Response};
use spin_sdk::{http_component, http_router};

pub mod cultures;
mod rand_utils;

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
        POST "/cultures/:culture/names" => api::names,
        _   "/*"             => |_req: Request, params| {
            let capture = params.wildcard().unwrap_or_default();
            Response::new(200, capture.to_string())
        }
    };
    router.handle(req)
}

mod api {
    use spin_sdk::http::{Params, Request, Response};

    use crate::{cultures::Culture, rand_utils};

    // /cultures/:culture/names
    pub fn names(_req: Request, params: Params) -> anyhow::Result<Response> {
        let culture = Culture::try_from(params.get("culture").expect("CULTURE"))?;
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
    use spin_sdk::http::Method;
    use strum::IntoEnumIterator;

    use crate::cultures::Culture;

    use super::*;

    #[test]
    fn returns_a_name() {
        for culture in Culture::iter() {
            let response = handle_loremaster(Request::new(
                Method::Post,
                format!("/cultures/{culture}/names"),
            ));
            assert_eq!(response.status(), &200u16);
        }
    }
}
