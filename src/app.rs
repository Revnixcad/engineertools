use crate::calculations::shapes_2d::*;
use crate::cards::area::{CircleCard, RectangleCard};
use crate::dom::storage::*;
use crate::locales::i18n::I18n;
use crate::pages::basics::BasicsPage;
use crate::pages::home::HomePage;
use components::{Route, Router, Routes};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    // add <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.css">
    // <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.js"></script>
    // to the head of the document
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>

                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() root=""/>
                <MetaTags/>
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.css"></link>
                <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.js"></script>
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
    let set_lang = use_context::<WriteSignal<String>>().expect("set_lang context not found");

    view! {
        <header class="header">
            <h1>engineertools.nl</h1>
            <div class ="language_toggle">
                <button on:click=move |_| set_lang.set("en".to_string())>"EN"</button>
                <button on:click=move |_| set_lang.set("nl".to_string())>"NL"</button>
            </div>
        </header>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn NavBar() -> impl IntoView {
    let path_is_active =
        move |path: &str| leptos_router::hooks::use_location().pathname.get() == path;

    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    view! {
        <div class="menu">
            <a class=move || {
                    if path_is_active("/") {
                        "menu__item--active"
                    } else {
                        "menu__item"
                    }
                } href="/">{move || i18n.get().t("menu_home").to_string()}</a>
            <a class=move || {
                    if path_is_active("/basics") {
                        "menu__item--active"
                    } else {
                        "menu__item"
                    }
                } href="/basics">{move || i18n.get().t("menu_basics").to_string()}</a>
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
                   <Route path=path!("/cards/area/circle") view=CircleCard/>
                   <Route path=path!("/cards/area/rectangle") view=RectangleCard/>
                   <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </Router>
        </main>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn PageFooter() -> impl IntoView {
    // Use the I18n context to get the current language

    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");
    view! {
        <footer class="footer">
            <small>{move || i18n.get().t("copyright").to_string()}</small>
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

    let (lang, set_lang) = signal("nl".to_string()); // Default to "nl"

    // Retrieve the stored language from localStorage on the browser
    Effect::new(move |_| {
        if let Some(stored_lang) = get_local_storage_item("lang") {
            set_lang.set(stored_lang);
        }
    });

    // Create a memoized I18n that updates when lang changes
    let i18n = Memo::new(move |_| I18n::new(&lang.get()));

    // Provide the memo as context
    provide_context(i18n);
    provide_context(set_lang);

    // when lang in i18n changes, update localStorage
    Effect::new(move |_| {
        let current_lang = lang.get();
        if let Err(e) = set_local_storage_item("lang", &current_lang) {
            eprintln!("Error setting localStorage item: {:?}", e);
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/engineertools.css"/>
        <Meta name="description" content=move || i18n.get().t("website_for_engineers").to_string()/>

        // <Title text="Welcome to engineertools.nl"/>
        <Title text=move || i18n.get().t("welcome_to_engineertools").to_string() />


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
