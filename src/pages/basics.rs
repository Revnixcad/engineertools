use crate::cards::sum::SumCard;
use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn BasicsContainer(children: Children) -> impl IntoView {
    view! {
        <div class="basic-container">{children()}</div>
    }
}

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
pub fn BasicsPage() -> impl IntoView {
    view! {
      <div class="basics">
        <p>"This page contains some basic calculations"</p>
        <BasicsContainer>
            <SumCard />
        </BasicsContainer>
      </div>
    }
}
