#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate maud;

use maud::*;
use std::collections::HashMap;
use stdweb::web::{
    window
};





pub trait View {
    fn view(&self) -> maud::Markup {
        html!{
            div {

            }
        }
    }
}

pub struct Router<'a> {
    pub routes: HashMap<String, &'a View>
}

impl <'a> Router<'a> {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new()
        }
    }

    pub fn add(&mut self, key: &str, component: &'a View) {
        self.routes.insert(
            key.to_string(),
            component,
        );
    }

    pub fn run(&self, mount_point: &str) {
        let href = window().location().unwrap().pathname().unwrap();
        if let Some(route) = self.routes.get(&href) {
            let rendered = route.view().into_string();
            js!{
                document.querySelector(@{mount_point}).innerHTML = @{rendered}
            }
        }
    }
}
