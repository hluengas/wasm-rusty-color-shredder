use wasm_bindgen::prelude::*;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use rand::Rng;

#[wasm_bindgen]
pub struct Canvas {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

#[wasm_bindgen]
impl Canvas {
    // constructor
    pub fn new(width: usize, height: usize) -> Canvas {
        let data: Vec<u8> = vec![0; (4 * width * height) as usize];

        Canvas {
            width,
            height,
            data,
        }
    }
}

#[wasm_bindgen]
pub fn render_canvas(
    canvas: &mut Canvas,
    context: &CanvasRenderingContext2d,
) -> Result<(), JsValue> {

    // test image
    populate_random_test_image(canvas);

    // create canvas ImageData from u8 vec
    let canvas_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&canvas.data),
        canvas.width as u32,
        canvas.height as u32,
    )?;
    // send to the canvas
    context.put_image_data(&canvas_image_data, 0.0, 0.0)
}

fn populate_random_test_image(canvas: &mut Canvas) {
    let mut rng = rand::thread_rng();

    for x in 0..canvas.width {
        for y in 0..canvas.height {
            for i in 0..4 {
                canvas.data[(x * 4) + (y * canvas.width * 4) + i] = rng.gen();
            }
        }
    }
}
