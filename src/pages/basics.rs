use crate::{
    cards::{
        area::{CircleCard, RectangleCard},
        sum::SumCard,
    },
    locales::i18n::I18n,
};
use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn BasicsContainer(children: Children) -> impl IntoView {
    view! {
        <div class="cards-container">{children()}</div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BasicsPage() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    view! {
        <div id="basic-page">
            <h1>{move || i18n.get().t("basic_page_title").to_string()}</h1>
            <p>{move || i18n.get().t("basic_page_description").to_string()}</p>
            <BasicsContainer>
                <SumCard />
                <CircleCard />
                <RectangleCard />
            </BasicsContainer>
        </div>
    }
}
