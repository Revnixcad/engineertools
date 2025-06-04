use leptos::prelude::*;

use crate::locales::i18n::I18n;

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    view! {
      <div class="home">
        <h1>{move || i18n.get().t("home_welcome").to_string()}</h1>
        <p>{move || i18n.get().t("home_description").to_string()}</p>
        <p>{move || i18n.get().t("home_resources").to_string()}</p>
      </div>
    }
}
