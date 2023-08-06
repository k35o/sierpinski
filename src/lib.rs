use wasm_bindgen::prelude::*;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&"Hello world!".into());

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    sierpinski(
        &context,
        Triangle {
            points: [
                Position { x: 300.0, y: 0.0 },
                Position { x: 0.0, y: 600.0 },
                Position { x: 600.0, y: 600.0 },
            ],
        },
        7,
    );
    Ok(())
}

#[derive(Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

struct Triangle {
    points: [Position; 3],
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: Triangle, depth: u32) {
    if depth == 0 {
        draw_triangle(&context, points);
        return;
    }
    let [top, left, right] = points.points;
    let left_middle = Position {
        x: (top.x + left.x) / 2.0,
        y: (top.y + left.y) / 2.0,
    };

    let right_middle = Position {
        x: (top.x + right.x) / 2.0,
        y: (top.y + right.y) / 2.0,
    };

    let bottom_middle = Position {
        x: (left.x + right.x) / 2.0,
        y: (left.y + right.y) / 2.0,
    };

    let top_triangle = Triangle {
        points: [top, left_middle, right_middle],
    };
    let left_triangle = Triangle {
        points: [left, left_middle.clone(), bottom_middle],
    };
    let right_triangle = Triangle {
        points: [right, right_middle.clone(), bottom_middle.clone()],
    };
    sierpinski(&context, top_triangle, depth - 1);
    sierpinski(&context, left_triangle, depth - 1);
    sierpinski(&context, right_triangle, depth - 1);
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: Triangle) {
    let [top, left, right] = points.points;
    context.move_to(top.x, top.y);
    context.begin_path();
    context.line_to(left.x, left.y);
    context.line_to(right.x, right.y);
    context.line_to(top.x, top.y);
    context.close_path();
    context.stroke();
}
