use crate::pages::basics::BasicsPage;
use crate::pages::home::HomePage;
use components::{Route, Router, Routes};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() root=""/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
        <header class="header">
            <h1>engineertools.nl</h1>
        </header>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn NavBar() -> impl IntoView {
    let path_is_active =
        move |path: &str| leptos_router::hooks::use_location().pathname.get() == path;

    view! {
        <div class="menu">
            <a class=move || {
                    if path_is_active("/") {
                        "menu__item--active"
                    } else {
                        "menu__item"
                    }
                } href="/">"Home"</a>
            <a class=move || {
                    if path_is_active("/basics") {
                        "menu__item--active"
                    } else {
                        "menu__item"
                    }
                } href="/basics">"Basics"</a>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Content() -> impl IntoView {
    let fallback = || view! { "Page not found." }.into_view();
    view! {
        <main class="content">
            <Router>
                <Routes fallback>
                   <Route path=path!("") view=HomePage/>
                   <Route path=path!("/basics") view=BasicsPage/>
                   <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        </main>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn PageFooter() -> impl IntoView {
    view! {
        <footer class="footer">
            <small>Copyright 2025 engineertools.nl</small>
        </footer>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! {
        <div class="container">{children()}</div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/engineertools.css"/>
        <Meta name="description" content="A website for engineers!"/>

        <Title text="Welcome engineertools.nl"/>

        <Container>
            <PageHeader/>
            <NavBar/>
            <Content/>
            <PageFooter/>
        </Container>
    }
}

/// 404 - Not Found
#[allow(non_snake_case)]
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        if let Some(resp) = use_context::<leptos_wasi::response::ResponseOptions>() {
            resp.set_status(leptos_wasi::prelude::StatusCode::NOT_FOUND);
        }
    }

    view! { <h1>"Not Found"</h1> }
}
