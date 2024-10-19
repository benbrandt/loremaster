/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
#![expect(clippy::needless_pass_by_value)]

use api::router;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod api;
mod characters;
mod cultures;
mod rand;

/// A simple Spin HTTP component.
#[http_component]
fn handle_loremaster(req: Request) -> Response {
    router(req)
}
