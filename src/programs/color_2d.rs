use super::super::common_functions;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

pub struct Color2D {
    program: WebGlProgram,
    vertex_array_len: usize,
    rectangle_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(webgl_context: &WebGlRenderingContext) -> Self {
        let program = common_functions::link_program(
            &webgl_context,
            super::super::shaders::vertex::color_2d::SHADER,
            super::super::shaders::fragment::color_2d::SHADER,
        )
        .unwrap();

        let rectangle_vertices: [f32; 12] =
            [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let rectangle_vertices_ptr = rectangle_vertices.as_ptr() as u32 / 4;

        let vertex_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            rectangle_vertices_ptr,
            rectangle_vertices_ptr + rectangle_vertices.len() as u32,
        );

        let vertex_array_len = rectangle_vertices.len();

        let rectangle_buffer = webgl_context
            .create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        webgl_context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&rectangle_buffer));
        webgl_context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        let u_color = webgl_context
            .get_uniform_location(&program, "uColor")
            .unwrap();
        let u_opacity = webgl_context
            .get_uniform_location(&program, "uOpacity")
            .unwrap();
        let u_transform = webgl_context
            .get_uniform_location(&program, "uTransform")
            .unwrap();

        Self {
            u_color: u_color,
            u_opacity: u_opacity,
            u_transform: u_transform,
            vertex_array_len: vertex_array_len,
            rectangle_buffer: rectangle_buffer,
            program: program,
        }
    }

    pub fn render(
        &self,
        webgl_context: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        webgl_context.use_program(Some(&self.program));

        webgl_context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.rectangle_buffer));
        webgl_context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        webgl_context.enable_vertex_attrib_array(0);

        webgl_context.uniform4f(
            Some(&self.u_color),
            0., //r
            0.5,//g
            0.5,//b
            1.0,//a
        );

        webgl_context.uniform1f(Some(&self.u_opacity), 1.);

        let translation_mat = common_functions::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_mat = common_functions::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );

        let transform_mat = common_functions::mult_matrix_4(scale_mat, translation_mat);
        webgl_context.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        webgl_context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (self.vertex_array_len / 2) as i32);
    }
}
