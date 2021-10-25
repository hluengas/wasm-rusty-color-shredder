use wasm_bindgen::prelude::*;
// use web_sys::*;
use web_sys::WebGlRenderingContext;

mod common_functions;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
pub struct Canvas {
    webgl_context: WebGlRenderingContext,
    webgl_program: programs::Color2D,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let webgl_context = gl_setup::initialize_webgl_contex().unwrap();

        Self {
            webgl_program: programs::Color2D::new(&webgl_context),
            webgl_context: webgl_context,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.webgl_context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        self.webgl_program.render(
            &self.webgl_context,
            0.0,  //bottom
            10.0, //top
            0.0,  //left
            10.0, //right
            10.0, //height
            10.0, //width
        );
    }
}
