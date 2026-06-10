use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");

    jsx! {
        <section className="page-section">
            <p className="section-label">"404"</p>
            <h1 className="page-title">"This page is not here."</h1>
            <div className="prose">
                <p>{format!("/{path}")}</p>
            </div>
        </section>
    }
}
