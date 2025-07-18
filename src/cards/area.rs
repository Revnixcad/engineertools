use leptos::prelude::*;

use crate::locales::i18n::I18n;
use leptos::wasm_bindgen::JsCast;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub fn create_rectangle_svg(rectangle: Rectangle) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%"><rect id="rectangle" x="{:.2}" y="{:.2}" style="fill:{};stroke:black;stroke-width:1px" width="{:.2}" height="{:.2}"/></svg>"#,
        10.0, // Centering the rectangle in the SVG
        10.0, // Centering the rectangle in the SVG
        "#3498db",
        rectangle.width,
        rectangle.height
    )
}

pub struct Circle {
    pub radius: f64,
}

// fn create svg of circle
pub fn create_circle_svg(circle: Circle) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%"><circle id="circle" cx="{:.2}" cy="{:.2}" style="fill:{};stroke:black;stroke-width:1px" r="{:.2}"/></svg>"#,
        circle.radius + 10.0, // Centering the circle in the SVG
        circle.radius + 10.0, // Centering the circle in the SVG
        "#3498db",
        circle.radius
    )
}

// create calulation error type
#[derive(Debug)]
pub struct CalculationError {
    pub message: String,
}

pub fn calculate_rectangle_area(rectangle: &Rectangle) -> Result<f64, CalculationError> {
    if rectangle.width < 0.0 || rectangle.height < 0.0 {
        return Err(CalculationError {
            message: "Width and height must be non-negative".to_string(),
        });
    }
    Ok(rectangle.width * rectangle.height)
}

pub fn calculate_circle_area(circle: &Circle) -> Result<f64, CalculationError> {
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
        let area = calculate_rectangle_area(&rect);
        sum_result.set(area.unwrap_or_else(|e| {
            leptos::logging::error!("Calculation error: {}", e.message);
            0.0 // Default to 0 on error
        }));

        // Render the latex formula dynamically
        let formula = format!(
            r"A = w \times h \\ A = {} \times {} \\ A = {}",
            rect.width,
            rect.height,
            sum_result.get()
        );
        if let Some(element) = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("rectangle-formula"))
        {
            let katex = js_sys::Reflect::get(&js_sys::global(), &"katex".into()).unwrap();
            let render = js_sys::Reflect::get(&katex, &"render".into())
                .unwrap()
                .dyn_into::<js_sys::Function>() // Convert JsValue to Function
                .expect("Expected katex.render to be a function");

            let _ = render.call2(&katex, &formula.into(), &element.into());
        }
    };

    view! {
      <div class="card">
        <a class="card__title">{move || i18n.get().t("rec_area_calculator").to_string()}</a>
        <div class="card__variables">
            <p>"A = "{move || i18n.get().t("rec_area_area").to_string()}</p>
            <p>"w = "{move || i18n.get().t("rec_area_breedte").to_string()}</p>
            <p>"h = "{move || i18n.get().t("rec_area_hoogte").to_string()}</p>
        </div>
        <div class="card__inputs">
          <input type="text" pattern="[0-9]*" bind:value=width />
          <input type="text" pattern="[0-9]*" bind:value=height />
          <button class="card__inputs__button" on:click=calculate_area>{move || i18n.get().t("rec_area_calculate").to_string()}</button>
        </div>
        <div class="card__result">
            <div class="card__result__svg">
                <div inner_html=move || create_rectangle_svg(Rectangle {
                    width: width.get().parse::<f64>().unwrap_or(0.0),
                    height: height.get().parse::<f64>().unwrap_or(0.0),
                }) />
            </div>
            <div id="rectangle-formula" class="card__result__formula"></div>
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
        let area = calculate_circle_area(&circ);
        sum_result.set(area.unwrap_or_else(|e| {
            leptos::logging::error!("Calculation error: {}", e.message);
            0.0 // Default to 0 on error
        }));
        // Render the latex formula dynamically
        let formula = format!(
            r"A = \pi r^2 \\ A = \pi \cdot {}^2 \\ A = {:.2}",
            circ.radius,
            sum_result.get()
        );
        if let Some(element) = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("circle-formula"))
        {
            let katex = js_sys::Reflect::get(&js_sys::global(), &"katex".into()).unwrap();
            let render = js_sys::Reflect::get(&katex, &"render".into())
                .unwrap()
                .dyn_into::<js_sys::Function>() // Convert JsValue to Function
                .expect("Expected katex.render to be a function");

            let _ = render.call2(&katex, &formula.into(), &element.into());
        }
    };

    let copy_to_clipboard = move |_| {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator(); // Access the Navigator object
            let clipboard = navigator.clipboard();
            let result_text = format!("{:.2}", sum_result.get());
            let _ = clipboard.write_text(&result_text);
            leptos::logging::log!("Copied to clipboard: {}", result_text);
        } else {
            leptos::logging::error!("Window object not available");
        }
    };

    view! {
      <div class="card">
        <a class="card__title">{move || i18n.get().t("circle_area_calculator").to_string()}</a>
        <div class="card__variables">
            <p>"A = "{move || i18n.get().t("rec_area_area").to_string()}</p>
            <p>"r = "{move || i18n.get().t("circle_area_radius").to_string()}</p>
        </div>
        <div class="card__inputs">
          <input type="text" pattern="[0-9]*" bind:value=radius />
          <button class="card__inputs__button" on:click=calculate_area>{move || i18n.get().t("circle_area_calculate").to_string()}</button>
        </div>
        <div class="card__result">
            <div class="card__result__svg">
                <div inner_html=move || create_circle_svg(Circle {
                    radius: radius.get().parse::<f64>().unwrap_or(0.0),
                }) />
            </div>
            <div id="circle-formula" class="card__result__formula"></div>
          <p>{move || i18n.get().t("circle_area_result").to_string()}{sum_result}</p>
          <button class="card__result__copy" on:click=copy_to_clipboard>{move || i18n.get().t("copy_to_clipboard").to_string()}</button>
        </div>
      </div>
    }
}
