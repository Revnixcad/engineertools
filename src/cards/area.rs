use leptos::prelude::*;

use crate::locales::i18n::I18n;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

// create calulation error type
#[derive(Debug)]
pub struct CalculationError {
    pub message: String,
}

pub fn calculate_rectangle_area(rectangle: Rectangle) -> Result<f64, CalculationError> {
    if rectangle.width < 0.0 || rectangle.height < 0.0 {
        return Err(CalculationError {
            message: "Width and height must be non-negative".to_string(),
        });
    }
    Ok(rectangle.width * rectangle.height)
}

pub fn calculate_circle_area(circle: Circle) -> Result<f64, CalculationError> {
    if circle.radius < 0.0 {
        return Err(CalculationError {
            message: "Radius must be non-negative".to_string(),
        });
    }
    Ok(std::f64::consts::PI * circle.radius * circle.radius)
}

/// Renders the home page of your application.
#[allow(non_snake_case)]
#[component]
pub fn RectangleCard() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    let sum_result = RwSignal::new(0.0);
    let width = RwSignal::new("0".to_string());
    let height = RwSignal::new("0".to_string());

    let calculate_area = move |_| {
        let rect = Rectangle {
            width: width.get().parse::<f64>().unwrap_or(0.0),
            height: height.get().parse::<f64>().unwrap_or(0.0),
        };
        let area = calculate_rectangle_area(rect);
        sum_result.set(area.unwrap_or_else(|e| {
            leptos::logging::error!("Calculation error: {}", e.message);
            0.0 // Default to 0 on error
        }));
    };

    view! {
      <div class="card">
        <a class="card__title">{move || i18n.get().t("rec_area_calculator").to_string()}</a>
        <div class="card__inputs">
          <input type="text" pattern="[0-9]*" bind:value=width />
          <input type="text" pattern="[0-9]*" bind:value=height />
          <button class="card__inputs__button" on:click=calculate_area>{move || i18n.get().t("rec_area_calculate").to_string()}</button>
        </div>
        <div class="card__result">
          <p>{move || i18n.get().t("rec_area_result").to_string()}{sum_result}</p>
        </div>
      </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn CircleCard() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    let sum_result = RwSignal::new(0.0);
    let radius = RwSignal::new("0".to_string());

    let calculate_area = move |_| {
        let circ = Circle {
            radius: radius.get().parse::<f64>().unwrap_or(0.0),
        };
        let area = calculate_circle_area(circ);
        sum_result.set(area.unwrap_or_else(|e| {
            leptos::logging::error!("Calculation error: {}", e.message);
            0.0 // Default to 0 on error
        }));
    };

    view! {
      <div class="card">
        <a class="card__title">{move || i18n.get().t("circle_area_calculator").to_string()}</a>
        <div class="card__inputs">
          <input type="text" pattern="[0-9]*" bind:value=radius />
          <button class="card__inputs__button" on:click=calculate_area>{move || i18n.get().t("circle_area_calculate").to_string()}</button>
        </div>
        <div class="card__result">
          <p>{move || i18n.get().t("circle_area_result").to_string()}{sum_result}</p>
        </div>
      </div>
    }
}
