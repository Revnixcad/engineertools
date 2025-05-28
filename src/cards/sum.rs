use leptos::prelude::*;

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
pub fn SumCard() -> impl IntoView {
    let sum_result = RwSignal::new(0);
    let num1 = RwSignal::new("0".to_string());
    let num2 = RwSignal::new("0".to_string());

    let calculate_sum = move |_| {
        let sum = num1.get().parse::<i32>().unwrap_or(0) + num2.get().parse::<i32>().unwrap_or(0);
        sum_result.set(sum);
    };

    view! {
      <div class="card">
      <h1>"Sum Calculator"</h1>

      <div class="card-inputs">
        <input type="text" pattern="[0-9]*" bind:value=num1 />
        <p>+</p>
        <input type="text" pattern="[0-9]*" bind:value=num2 />
        <button on:click=calculate_sum>"Calculate Sum"</button>
      </div>

      <div class="card-result">
        <p>"The sum is: " {sum_result}</p>
      </div>

      </div>
    }
}
