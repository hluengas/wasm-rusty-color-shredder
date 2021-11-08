use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
use web_sys::*;

pub fn initialize_webgl_contex() -> Result<WebGlRenderingContext, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("html_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let webgl_context: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    webgl_context.enable(WebGlRenderingContext::BLEND);
    webgl_context.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
    webgl_context.clear_color(0.0, 0.0, 0.0, 1.0);
    webgl_context.clear_depth(1.0);

    return Ok(webgl_context);
}
