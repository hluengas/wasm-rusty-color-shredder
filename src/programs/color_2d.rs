use wasm_bindgen::JsCast;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlRenderingContext;
use js_sys::WebAssembly;
use super::super::common_functions;

pub struct Color2D {
    program: WebGlProgram,
}

impl Color2D {
    pub fn new(webgl_context: &WebGlRenderingContext) -> Self {
        let program = common_functions::link_program(
            &webgl_context,
            super::super::shaders::vertex::color_2d::SHADER,
            super::super::shaders::fragment::color_2d::SHADER,
        ).unwrap();

    let rectangle_vertices: [f32; 12] = [
        0.0,1.0,
        0.0,0.0,
        1.0,1.0,

        1.0,1.0,
        0.0,0.0,
        1.0,0.0,
    ];

    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    
    let rectangle_vertices_ptr = rectangle_vertices.as_ptr() as u32 / 4;
    
    let vertex_array = js_sys::Float32Array::new(&memory_buffer).subarray(
        rectangle_vertices_ptr,
        rectangle_vertices_ptr + rectangle_vertices.len() as u32,
    );

    let rectangle_buffer = webgl_context.create_buffer().ok_or("failed to create buffer").unwrap();
    webgl_context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&rectangle_buffer));

    Self {
        program: program,

    }
}