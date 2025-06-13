use leptos::{logging::log, prelude::*};

use crate::locales::i18n::I18n;

// create calulation error type
#[derive(Debug)]
pub struct CalculationError {
    pub message: String,
}

pub fn calculate_sum(num1: i32, num2: i32, operator: &str) -> Result<i32, CalculationError> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "x" | "X" | "*" => Ok(num1 * num2),
        "/" => {
            if num2 != 0 {
                Ok(num1 / num2)
            } else {
                Ok(0)
            }
        } // Avoid division by zero
        _ => Err(CalculationError {
            message: format!("{} is not a supported operator", operator),
        }),
    }
}

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
pub fn SumCard() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    let sum_result = RwSignal::new(0);
    let num1 = RwSignal::new("0".to_string());
    let num2 = RwSignal::new("0".to_string());
    let operator = RwSignal::new("+".to_string());

    let calculate_sum = move |_| {
        let sum = calculate_sum(
            num1.get().parse::<i32>().unwrap_or(0),
            num2.get().parse::<i32>().unwrap_or(0),
            operator.get().as_str(),
        );
        sum_result.set(sum.unwrap_or_else(|e| {
            leptos::logging::error!("Calculation error: {}", e.message);
            0 // Default to 0 on error
        }));
    };

    view! {
      <div class="card">
        <a class="card__title">{move || i18n.get().t("sum_calculator").to_string()}</a>
        <div class="card__inputs">
          <input type="text" pattern="[0-9]*" bind:value=num1 />
          <input type="text" pattern="[0-9]*" bind:value=operator />
          <input type="text" pattern="[0-9]*" bind:value=num2 />
          <button class="card__inputs__button" on:click=calculate_sum>{move || i18n.get().t("calculate_sum").to_string()}</button>
        </div>
        <div class="card__result">
          <p>{move || i18n.get().t("sum_result").to_string()}{sum_result}</p>
        </div>
      </div>
    }
}
