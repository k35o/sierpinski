use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
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
            color: rdn_color(),
        },
        6,
    );
    Ok(())
}

#[derive(Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Copy)]
struct Triangle {
    points: [Position; 3],
    color: Color,
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: Triangle, depth: u32) {
    draw_triangle(&context, points);
    if depth == 0 {
        return;
    }
    let [top, left, right] = points.points.clone();

    let next_color = rdn_color();
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
        color: next_color,
    };
    let left_triangle = Triangle {
        points: [left, left_middle.clone(), bottom_middle],
        color: next_color.clone(),
    };
    let right_triangle = Triangle {
        points: [right, right_middle.clone(), bottom_middle.clone()],
        color: next_color.clone(),
    };
    sierpinski(&context, top_triangle, depth - 1);
    sierpinski(&context, left_triangle, depth - 1);
    sierpinski(&context, right_triangle, depth - 1);
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: Triangle) {
    let [top, left, right] = points.points;
    context.set_fill_style(&JsValue::from_str(&format!(
        "rgb({}, {}, {})",
        points.color.r, points.color.g, points.color.b
    )));
    context.move_to(top.x, top.y);
    context.begin_path();
    context.line_to(left.x, left.y);
    context.line_to(right.x, right.y);
    context.line_to(top.x, top.y);
    context.close_path();
    context.stroke();
    context.fill();
}

fn rdn_color() -> Color {
    let mut rng = thread_rng();
    Color {
        r: rng.gen_range(0..255),
        g: rng.gen_range(0..255),
        b: rng.gen_range(0..255),
    }
}
