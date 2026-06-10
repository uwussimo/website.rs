use crate::components::ArticleList;
use crate::Route;
use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn Home() -> Element {
    jsx! {
        <section className="hero">
            <h1>
                <span>"building products"</span>
                <span>"that people love."</span>
            </h1>
            <p>"hey, i'm Yusuf, a builder & engineer sharing lessons from many startups i have founded & built, so you don't have to make the same mistakes."</p>
            <div className="actions">
                <Link className="button button-primary" to={Route::Essays {}}>"Read Essays"</Link>
                <Link className="button button-secondary" to={Route::Projects {}}>"View Projects"</Link>
            </div>
        </section>
        <section className="latest">
            <p className="section-label">"LATEST"</p>
            <ArticleList limit={Some(3)} />
        </section>
    }
}
