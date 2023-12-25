mod drawable;
mod math;
mod utils;

pub use drawable::Drawable;

use math::{Graph, Point};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let window = web_sys::window().expect("Could not acquire window");
    let document = window.document().expect("Could not acquire document");

    let canvas = document
        .get_element_by_id("canvas")
        .expect("Could not acquire canvas")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Node with id \"canvas\" should be of type HtmlCanvasElement");

    canvas.set_width(600);
    canvas.set_height(600);

    let ctx = canvas
        .get_context("2d")?
        .expect("Could not acquire 2d canvas context")
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut graph = Graph::default();

    let p1 = Point::new(200.0, 200.0);
    let p2 = Point::new(500.0, 200.0);
    let p3 = Point::new(400.0, 400.0);
    let p4 = Point::new(100.0, 300.0);

    graph.add_points(vec![&p1, &p2, &p3, &p4]);

    graph.create_segment(&p1, &p2);
    graph.create_segment(&p1, &p3);
    graph.create_segment(&p1, &p4);
    graph.create_segment(&p2, &p3);

    graph.draw(&ctx)
}
