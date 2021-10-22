use wasm_bindgen::prelude::*;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use rand::Rng;

#[wasm_bindgen]
pub fn main(
    canvas_context: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {
    // generate a test image (u8 vec)
    let mut canvas_data = create_test_image(width, height);
    // create canvas ImageData from u8 vec
    let canvas_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut canvas_data), width, height)?;
    // send to the canvas
    canvas_context.put_image_data(&canvas_data, 0.0, 0.0)
}

fn create_test_image(width: u32, height: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for _ in 0..width {
        for _ in 0..height {
            let r_value: u8 = rng.gen();
            let g_value: u8 = rng.gen();
            let b_value: u8 = rng.gen();
            let a_value: u8 = 255;

            data.push(r_value);
            data.push(g_value);
            data.push(b_value);
            data.push(a_value);
        }
    }

    data
}
