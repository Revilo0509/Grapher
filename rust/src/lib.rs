mod parser;

use std::collections::HashMap;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{console, CanvasRenderingContext2d, ImageData};

use crate::parser::Equation;

const LINE_SPACING: u32 = 100;

const WHITE: [u8; 4] = [255, 255, 255, 255];

fn set_pixel(data: &mut [u8], i: usize, color: [u8; 4]) {
    data[i] = color[0];
    data[i + 1] = color[1];
    data[i + 2] = color[2];
    data[i + 3] = color[3];
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    equations: Vec<String>,
) -> Result<(), JsValue> {
    let equations_objects: Vec<Equation> = equations
        .iter()
        .map(|e| Equation::new(e.to_owned()))
        .collect();

    let knowns: HashMap<String, f64> = HashMap::new();
    let eq = Equation::new("2*x+3=7".to_string());
    if let Some(result) = eq.solve(&knowns, "x", -100.0, 100.0) {
        console::log_1(&result.into());
    }

    let mut data = vec![0u8; (width * height * 4) as usize]; // RGBA
    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 4) as usize;
            if x % LINE_SPACING == 0 {
                set_pixel(&mut data, i, WHITE);
            }
            if y % LINE_SPACING == 0 {
                set_pixel(&mut data, i, WHITE);
            }
        }
    }
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    ctx.put_image_data(&image_data, 0.0, 0.0)
}
