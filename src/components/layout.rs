use crate::Route;
use dioxus::prelude::*;
use jsxish::jsx;

#[component]
pub fn Layout() -> Element {
    let mut is_dark = use_signal(|| false);
    let theme_class = if is_dark() { "page dark" } else { "page light" };
    let theme_icon = if is_dark() { "☾" } else { "☼" };

    jsx! {
        <div className={theme_class}>
            <header className="site-header">
                <nav className="nav">
                    <Link className="icon-link" to={Route::Home {}} aria_label="home">"⌂"</Link>
                    <Link to={Route::About {}}>"about"</Link>
                    <Link to={Route::Essays {}}>"essays"</Link>
                    <Link to={Route::Projects {}}>"projects"</Link>
                    <button
                        className="theme-toggle"
                        type="button"
                        aria_label="toggle theme"
                        onclick={move |_| is_dark.toggle()}
                    >
                        {theme_icon}
                    </button>
                </nav>
            </header>
            <main className="content">
                <RouteOutlet />
            </main>
        </div>
    }
}

#[component]
fn RouteOutlet() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
