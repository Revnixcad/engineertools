use crate::calculations::errors::*;
use crate::calculations::shapes_2d::*;
use crate::dom::clipboard::float_to_clipboard;
use crate::locales::i18n::I18n;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

pub fn create_rectangle_svg(rectangle: Rectangle) -> String {
    let x_position = rectangle.width / 2.0; // Center horizontally
    let y_position = rectangle.height / 2.0; // Center vertically

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg">
            <rect id="rectangle" x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" />
        </svg>"#,
        // Rectangle position
        x_position,
        y_position,
        // Rectangle dimensions
        rectangle.width,
        rectangle.height
    )
}

// fn create svg of circle
pub fn create_circle_svg(circle: Circle) -> String {
    let cx = circle.radius + 10.0; // Centering the circle in the SVG
    let cy = circle.radius + 10.0; // Centering the circle in the SVG
    let dimension_end_x = cx + circle.radius; // End of the dimension line

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg">
            <circle id="circle" cx="{:.2}" cy="{:.2}" r="{:.2}" />
            <g id="dimension">
                <circle cx="{:.2}" cy="{:.2}" r="3" />
                <text x="{:.2}" y="{:.2}" font-size="12" text-anchor="middle" dominant-baseline="middle">R</text>
                <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" />
                <circle cx="{:.2}" cy="{:.2}" r="3" />
            </g>
        </svg>"#,
        cx,
        cy,            // Circle center
        circle.radius, // Circle radius
        cx,
        cy, // Start of the dimension line
        dimension_end_x + 5.0,
        cy, // Position of the "R" text
        cx,
        cy,
        dimension_end_x,
        cy, // Dimension line
        dimension_end_x,
        cy // End of the dimension line
    )
}

#[allow(non_snake_case)]
#[component]
pub fn RectangleCard() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    let sum_result = RwSignal::new(0.0);
    let width = RwSignal::new("w".to_string());
    let height = RwSignal::new("h".to_string());

    let calculate_area = move |_| {
        let rect = Rectangle::new(
            width.get().parse::<f64>().unwrap_or(0.0),
            height.get().parse::<f64>().unwrap_or(0.0),
        )
        .unwrap_or_default();
        let area = Geometric2D::area(&rect);
        sum_result.set(area);

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

    let result_to_clipboard = float_to_clipboard(sum_result.get());

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
          <a href="/cards/area/rectangle">{move || i18n.get().t("card_direct_link").to_string()}</a>
          <button on:click=result_to_clipboard>{move || i18n.get().t("copy_to_clipboard").to_string()}</button>
        </div>
      </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn CircleCard() -> impl IntoView {
    let i18n = use_context::<Memo<I18n>>().expect("I18n context not found");

    let sum_result = RwSignal::new(0.0);
    let radius = RwSignal::new("r".to_string());

    let calculate_area = move |_| {
        let circ = Circle::new(radius.get().parse::<f64>().unwrap_or(0.0)).unwrap_or_default();
        let area = Geometric2D::area(&circ);
        sum_result.set(area);
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

    let result_to_clipboard = float_to_clipboard(sum_result.get());

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
          <a href="/cards/area/circle">{move || i18n.get().t("card_direct_link").to_string()}</a>
          <button on:click=result_to_clipboard>{move || i18n.get().t("copy_to_clipboard").to_string()}</button>
        </div>
      </div>
    }
}
