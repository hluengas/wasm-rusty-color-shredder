use super::super::common_functions;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

pub struct Color2DGradient {
    program: WebGlProgram,
    rectangle_color_buffer: WebGlBuffer,
    rectangle_vertex_buffer: WebGlBuffer,
    rectangle_index_count: i32,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2DGradient {
    pub fn new(webgl_context: &WebGlRenderingContext) -> Self {
        // create program
        let program = common_functions::link_program(
            &webgl_context,
            super::super::shaders::vertex::color_2d_gradient::SHADER,
            super::super::shaders::fragment::color_2d_gradient::SHADER,
        )
        .unwrap();

        // create & fill rectangle vertex buffer
        let rectangle_vertex_buffer: web_sys::WebGlBuffer = new_vertex_buffer(&webgl_context);
        // create & fill rectangle index buffer
        let rectangle_index_count: i32 = new_index_buffer(&webgl_context);
        // create rectangle color buffer (not filled)
        let rectangle_color_buffer: web_sys::WebGlBuffer = new_color_buffer(&webgl_context);

        // get uniform pointers
        let u_opacity = webgl_context
            .get_uniform_location(&program, "uOpacity")
            .unwrap();
        let u_transform = webgl_context
            .get_uniform_location(&program, "uTransform")
            .unwrap();

        // instantiate
        Self {
            // uniforms
            u_opacity: u_opacity,
            u_transform: u_transform,

            // buffers
            rectangle_vertex_buffer: rectangle_vertex_buffer,
            rectangle_color_buffer: rectangle_color_buffer,

            // counts
            rectangle_index_count: rectangle_index_count,

            // program
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
        // enable program
        webgl_context.use_program(Some(&self.program));

        // set attributes for and enable rectangle vertex buffer
        webgl_context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.rectangle_vertex_buffer),
        );
        webgl_context.vertex_attrib_pointer_with_i32(
            0,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        webgl_context.enable_vertex_attrib_array(0);

        // set attributes for and enable rectangle color buffer
        webgl_context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.rectangle_color_buffer),
        );
        webgl_context.vertex_attrib_pointer_with_i32(
            1,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        webgl_context.enable_vertex_attrib_array(1);

        // send opacity uniform
        webgl_context.uniform1f(Some(&self.u_opacity), 0.5);

        // create transform matrix uniform
        let transform_mat = get_transform_from_canvas_dimensions(
            bottom,
            top,
            left,
            right,
            canvas_height,
            canvas_width,
        );

        // send transform matrix uniform
        webgl_context.uniform_matrix4fv_with_f32_array(
            Some(&self.u_transform),
            false,
            &transform_mat,
        );

        // webgl draw call
        webgl_context.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            self.rectangle_index_count,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

fn new_vertex_buffer(webgl_context: &WebGlRenderingContext) -> web_sys::WebGlBuffer {
    // define rectangle vertices
    let rectangle_vertex_array: [f32; 8] = [
        0.0, 1.0, // x, y
        0.0, 0.0, // x, y
        1.0, 1.0, // x, y
        1.0, 0.0, // x, y
    ];
    // allocate memory buffer
    let vertex_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    // get pointer to vertex array
    let rectangle_vertex_array_ptr = rectangle_vertex_array.as_ptr() as u32 / 4;
    // put vertex array into web_gl format
    let webgl_vertex_array = js_sys::Float32Array::new(&vertex_memory_buffer).subarray(
        rectangle_vertex_array_ptr,
        rectangle_vertex_array_ptr + rectangle_vertex_array.len() as u32,
    );
    // create webgl buffer
    let rectangle_vertex_buffer = webgl_context
        .create_buffer()
        .ok_or("failed to create buffer")
        .unwrap();
    // bind buffer
    webgl_context.bind_buffer(
        WebGlRenderingContext::ARRAY_BUFFER,
        Some(&rectangle_vertex_buffer),
    );
    // fill buffer
    webgl_context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &webgl_vertex_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    return rectangle_vertex_buffer;
}

fn new_index_buffer(webgl_context: &WebGlRenderingContext) -> i32 {
    // define rectangle triangle vertex indicies
    let rectangle_index_array: [u16; 6] = [0, 1, 2, 2, 1, 3];
    // allocate memory buffer
    let indices_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    // get pointer to index array
    let rectangle_index_array_ptr = rectangle_index_array.as_ptr() as u32 / 2;
    // put index array into web_gl format
    let webgl_index_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
        rectangle_index_array_ptr,
        rectangle_index_array_ptr + rectangle_index_array.len() as u32,
    );
    let rectangle_index_count = webgl_index_array.length() as i32;
    // create webgl buffer
    let rectangle_index_buffer = webgl_context.create_buffer().unwrap();
    // bind buffer
    webgl_context.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&rectangle_index_buffer),
    );
    // fill buffer
    webgl_context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        &webgl_index_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    return rectangle_index_count;
}

fn new_color_buffer(webgl_context: &WebGlRenderingContext) -> web_sys::WebGlBuffer {
    // define rectangle triangle vertex indicies
    let colors: [f32; 16] = [
        1.0, 0.0, 0.0, 1.0, //rgba
        0.0, 1.0, 0.0, 1.0, //rgba
        0.0, 0.0, 1.0, 1.0, //rgba
        1.0, 1.0, 1.0, 1.0, //rgba
    ];
    // allocate memory buffer
    let colors_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    // get pointer to color array
    let color_vals_location = colors.as_ptr() as u32 / 4;
    // put color array into web_gl format
    let color_vals_array = js_sys::Float32Array::new(&colors_memory_buffer).subarray(
        color_vals_location,
        color_vals_location + colors.len() as u32,
    );
    // create webgl buffer
    let rectangle_color_buffer = webgl_context
        .create_buffer()
        .ok_or("failed to create buffer")
        .unwrap();

    // bind buffer
    webgl_context.bind_buffer(
        WebGlRenderingContext::ARRAY_BUFFER,
        Some(&rectangle_color_buffer),
    );
    // fill buffer
    webgl_context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &color_vals_array,
        WebGlRenderingContext::DYNAMIC_DRAW,
    );

    return rectangle_color_buffer;
}

fn get_transform_from_canvas_dimensions(
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
) -> [f32; 16] {
    let translation_mat = common_functions::translation_matrix(
        2.0 * left / canvas_width - 1.0,
        2.0 * bottom / canvas_height - 1.0,
        0.0,
    );
    let scale_mat = common_functions::scaling_matrix(
        2.0 * (right - left) / canvas_width,
        2.0 * (top - bottom) / canvas_height,
        0.0,
    );
    let transform_mat = common_functions::mult_matrix_4(scale_mat, translation_mat);

    return transform_mat;
}
