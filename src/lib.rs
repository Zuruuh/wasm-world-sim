mod drawable;
mod math;
mod utils;

use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
};

pub use drawable::Drawable;

use math::{Graph, Point, PointId};
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlButtonElement, HtmlCanvasElement, HtmlDivElement,
};

#[wasm_bindgen(main)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log::init_with_level(log::Level::Debug).unwrap();

    let window = web_sys::window().expect("Could not acquire window");
    let document = window.document().expect("Could not acquire document");
    let controls = document
        .get_element_by_id("controls")
        .expect("Could not acquire canvas")
        .dyn_into::<HtmlDivElement>()
        .expect("Node with id \"controls\" should be of type HtmlDivElement");

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

    let graph = Rc::new(Graph::new(ctx));

    {
        let mut graph = graph.clone();
        let p1 = graph.add_point(Point::new(200.0, 200.0));
        let p2 = graph.add_point(Point::new(500.0, 200.0));
        let p3 = graph.add_point(Point::new(400.0, 400.0));
        let p4 = graph.add_point(Point::new(100.0, 300.0));

        graph.create_segment(p1, p2);
        // graph.create_segment(&P1, &P3);
        // graph.create_segment(&P1, &P4);
        // graph.create_segment(&P2, &P3);
    }

    // setup_controls(&document, &controls, &mut graph)?;

    let add_random_button = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;

    add_random_button.set_text_content(Some("Add random node"));

    // {
    //     let mut graph = graph.clone();
    //     let callback = Closure::<dyn FnMut()>::new(move || {
    //         graph.create_segment(PointId(1), PointId(3)).unwrap();
    //     });
    //
    //     add_random_button.set_onclick(Some(callback.as_ref().unchecked_ref()));
    //     callback.forget();
    // }

    controls.append_child(&add_random_button)?;

    Ok(())
}

// fn setup_controls(
//     document: &Document,
//     controls: &HtmlDivElement,
//     graph: &mut Graph,
// ) -> Result<(), JsValue> {
//     let add_random_button = document
//         .create_element("button")?
//         .dyn_into::<HtmlButtonElement>()?;
//
//     add_random_button.set_text_content(Some("Add random node"));
//
//     let callback = Closure::<dyn FnMut()>::new(move || {
//         graph.create_segment(PointId(1), PointId(3)).unwrap();
//     });
//
//     add_random_button.set_onclick(Some(callback.as_ref().unchecked_ref()));
//     callback.forget();
//
//     controls.append_child(&add_random_button)?;
//
//     Ok(())
// }
