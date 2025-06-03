use leptos::prelude::*;

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
      <div class="home">
        <h1>"Welcome to engineertools.nl"</h1>
        <p>"This is a website for engineers, built with Leptos!"</p>
        <p>"You can use this website to find useful tools and resources for your engineering projects."</p>
      </div>
    }
}
