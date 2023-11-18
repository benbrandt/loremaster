/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
#![allow(clippy::needless_pass_by_value)]

use spin_sdk::http::{Request, Response};
use spin_sdk::{http_component, http_router};

pub mod names;
mod rand_utils;

/// A simple Spin HTTP component.
#[http_component]
fn handle_loremaster(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let router = http_router! {
        GET "/names/:culture" => api::names,
        _   "/*"             => |_req: Request, params| {
            let capture = params.wildcard().unwrap_or_default();
            Response::new(200, capture.to_string())
        }
    };
    router.handle(req)
}

mod api {
    use spin_sdk::http::{Params, Request, Response};

    use crate::{names::Name, rand_utils};

    // /names/:culture
    pub fn names(_req: Request, params: Params) -> anyhow::Result<Response> {
        let culture = Name::try_from(params.get("culture").expect("CULTURE"))?;
        let mut rng = rand_utils::rng_from_entropy();
        let name = culture.gen(&mut rng).to_string();

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

    use crate::names::Name;

    use super::*;

    #[test]
    fn returns_a_name() {
        for culture in Name::iter() {
            let response =
                handle_loremaster(Request::new(Method::Get, format!("/names/{culture}")));
            assert_eq!(response.status(), &200u16);
        }
    }
}
