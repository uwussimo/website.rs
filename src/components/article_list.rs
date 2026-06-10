use crate::data::POSTS;
use crate::Route;
use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn ArticleList(limit: Option<usize>) -> Element {
    let limit = limit.unwrap_or(POSTS.len());

    jsx! {
        <div className="post-list">
            {
                POSTS.iter().take(limit).map(|post| {
                    jsx! {
                        <Link className="post" to={Route::Post { slug: post.slug.to_string() }}>
                            <span className="post-title">{post.title}</span>
                            <span className="post-meta">
                                <span>{post.date}</span>
                                <span>{post.minutes}</span>
                            </span>
                        </Link>
                    }
                })
            }
        </div>
    }
}
