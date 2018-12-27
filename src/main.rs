#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate beans;
extern crate stdweb;
#[macro_use]
extern crate maud;
use stdweb::*;

use beans::{Router, View};

macro_rules! routs {
    ( $element:expr, =, $($route:expr, =, $component:expr)+ ) => {{
        
        let router = beans::Router::new();

        $(
            router.add($route, $component);
        )+

        router.run($element);
    }}
}

struct home;
struct about;

impl View for home {
    fn view(&self) -> maud::Markup {
        html!{
            h1 {
                "Helo, World!"
            }
        }
    }
}

impl View for about {
    fn view(&self) -> maud::Markup {
        html!{
            h1 {
                "About"
            }
            h2 {
                "Stuff"
            }
        }
    }
}

fn main() {
    console!(log, "Hello, World!");
    routs! {
        "#app" =
            "/" = home
    };
}
