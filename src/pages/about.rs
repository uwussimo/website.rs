use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn About() -> Element {
    jsx! {
        <section className="page-section">
            <p className="section-label">"ABOUT"</p>
            <h1 className="page-title">"Builder, engineer, and founder focused on useful products."</h1>
            <div className="prose">
                <p>"I write about the operating lessons behind product work: how teams make decisions, how software gets shipped, and how builders keep taste while moving fast."</p>
                <p>"This site is intentionally small. Essays first, projects second, no marketing wrapper."</p>
            </div>
        </section>
    }
}
