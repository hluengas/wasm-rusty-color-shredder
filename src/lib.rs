use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

mod app_state;
mod common_functions;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
pub struct Canvas {
    webgl_context: WebGlRenderingContext,
    webgl_program_color_2d_gradient: programs::Color2DGradient,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let webgl_context = gl_setup::initialize_webgl_contex().unwrap();

        Self {
            webgl_program_color_2d_gradient: programs::Color2DGradient::new(&webgl_context),
            webgl_context: webgl_context,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        return Ok(());
    }

    pub fn render(&self) {
        self.webgl_context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        let current_state = app_state::get_current_state();

        self.webgl_program_color_2d_gradient.render(
            &self.webgl_context,
            current_state.control_bottom,
            current_state.control_top,
            current_state.control_left,
            current_state.control_right,
            current_state.canvas_height,
            current_state.canvas_width,
        );
    }
}
