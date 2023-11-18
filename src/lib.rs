/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

pub mod names;
mod rand_utils;

/// A simple Spin HTTP component.
#[http_component]
#[allow(clippy::needless_pass_by_value)]
fn handle_loremaster(req: Request) -> impl IntoResponse {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build()
}
