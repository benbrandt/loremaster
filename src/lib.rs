/*!
# loremaster

A collection of tools to help a Loremaster running a campaign with The One Ring 2E.
*/
#![allow(clippy::needless_pass_by_value)]

use api::{router, Generator};
use bindings::loremaster::{
    characters::{
        generate::generate_character,
        types::{Character, HeroicCulture},
    },
    cultures::generate::generate_name,
};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod api;
#[allow(warnings)]
mod bindings;

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
    router::<Component>(req)
}
