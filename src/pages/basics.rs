use crate::cards::sum::SumCard;
use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn BasicsContainer(children: Children) -> impl IntoView {
    view! {
        <div>{children()}</div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BasicsPage() -> impl IntoView {
    view! {
        <p>"This page contains some basic calculations"</p>
        <BasicsContainer>
            <SumCard />
        </BasicsContainer>
    }
}
