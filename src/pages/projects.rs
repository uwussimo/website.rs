use crate::data::PROJECTS;
use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn Projects() -> Element {
    jsx! {
        <section className="page-section">
            <p className="section-label">"PROJECTS"</p>
            <h1 className="page-title">"Current products and experiments."</h1>
            <div className="project-list">
                {
                    PROJECTS.iter().map(|project| {
                        jsx! {
                            <article className="project">
                                <div>
                                    <h2>{project.name}</h2>
                                    <p>{project.summary}</p>
                                </div>
                                <span>{project.status}</span>
                            </article>
                        }
                    })
                }
            </div>
        </section>
    }
}
