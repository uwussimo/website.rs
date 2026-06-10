use crate::data::post_by_slug;
use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn Post(slug: String) -> Element {
    let Some(post) = post_by_slug(&slug) else {
        return jsx! {
            <section className="page-section">
                <p className="section-label">"NOT FOUND"</p>
                <h1 className="page-title">"That essay does not exist."</h1>
            </section>
        };
    };

    jsx! {
        <article className="article">
            <p className="section-label">"ESSAY"</p>
            <h1 className="article-title">{post.title}</h1>
            <div className="article-meta">
                <span>{post.date}</span>
                <span>{post.minutes}</span>
            </div>
            <p className="article-excerpt">{post.excerpt}</p>
            <div className="article-body">
                {
                    post.body.iter().map(|paragraph| {
                        jsx! {
                            <p>{*paragraph}</p>
                        }
                    })
                }
            </div>
        </article>
    }
}
