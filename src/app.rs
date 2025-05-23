use components::{Route, Router, Routes};
use leptos::{prelude::*, task::spawn_local};
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
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Stylesheet id="leptos" href="/pkg/engineertools.css"/>
        <Meta name="description" content="A website for engineers!"/>

        <Title text="Welcome engineertools.nl"/>

        <Router>
            <main>
                <Routes fallback>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
        spawn_local(async move {
            save_count(count.get()).await.unwrap();
        });
    };

    view! {
      <h1>"Welcome to engineering.nl!"</h1>
      <button on:click=on_click>"Click Me: " {count}</button>
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

#[server(prefix = "/api")]
pub async fn save_count(count: u32) -> Result<(), ServerFnError<String>> {
    println!("Saving value {count}");
    let store = spin_sdk::key_value::Store::open_default().map_err(|e| e.to_string())?;
    store
        .set_json("engineertools_count", &count)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
