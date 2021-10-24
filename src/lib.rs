use wasm_bindgen::prelude::*;
// use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod shaders;
mod programs;
mod common_functions;

#[wasm_bindgen]
pub struct Canvas {
    webgl_context: GL,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let webgl_context = gl_setup::initialize_webgl_contex().unwrap();

        Self {
            webgl_context,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.webgl_context.clear(GL::COLOR_BUFFER_BIT | GL :: DEPTH_BUFFER_BIT);
    }
}
