use crate::components::{Echo, Hero};
use dioxus::prelude::*;
use jsxish::jsx;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    jsx! {
        <Hero />
        <Echo />
    }
}
