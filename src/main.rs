use dioxus::prelude::*;
use jsxish::jsx;

use components::Layout;
use pages::{About, Essays, Home, NotFound, Post, Projects};

mod components;
mod data;
mod pages;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/essays")]
    Essays {},
    #[route("/essays/:slug")]
    Post { slug: String },
    #[route("/projects")]
    Projects {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    jsx! {
        <document::Link rel="icon" href={FAVICON} />
        <document::Link rel="stylesheet" href={MAIN_CSS} />
        <RouteRouter />
    }
}

#[component]
fn RouteRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
