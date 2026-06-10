use dioxus::prelude::*;
use jsxish::jsx;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    jsx! {
        <div id="hero">
            {"Fuck you mother fucker!"}
        </div>
    }
}
