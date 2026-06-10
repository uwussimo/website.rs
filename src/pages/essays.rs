use crate::components::ArticleList;
use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn Essays() -> Element {
    jsx! {
        <section className="page-section">
            <p className="section-label">"ESSAYS"</p>
            <h1 className="page-title">"Notes on building, shipping, and choosing better constraints."</h1>
            <ArticleList limit={None} />
        </section>
    }
}
