use super::super::common_functions;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

pub struct Graph3D {
    pub program: WebGlProgram,
    pub indices_buffer: WebGlBuffer,
    pub index_count: i32,
    pub position_buffer: WebGlBuffer,
    pub u_normals_rotation: WebGlUniformLocation,
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
    pub y_buffer: WebGlBuffer,
}

impl Graph3D {
    pub fn new(webgl_context: &WebGlRenderingContext) -> Self {
        let program = common_functions::link_program(
            &webgl_context,
            &super::super::shaders::vertex::graph_3d::SHADER,
            &super::super::shaders::fragment::varying_color_from_vertex::SHADER,
        )
        .unwrap();

        let (position_buffer, indices_buffer, index_count): (
            web_sys::WebGlBuffer,
            web_sys::WebGlBuffer,
            i32,
        ) = create_grid_buffers(&webgl_context);

        Self {
            u_opacity: webgl_context
                .get_uniform_location(&program, "uOpacity")
                .unwrap(),
            u_projection: webgl_context
                .get_uniform_location(&program, "uProjection")
                .unwrap(),
            program: program,

            position_buffer: position_buffer,
            indices_buffer: indices_buffer,
            index_count: index_count,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_angle_x_axis: f32,
        rotation_angle_y_axis: f32,
        y_vals: &Vec<f32>,
    ) {
        gl.use_program(Some(&self.program));

        let my_3d_matrices = common_functions::get_3d_matrices(
            bottom,
            top,
            left,
            right,
            canvas_height,
            canvas_width,
            rotation_angle_x_axis,
            rotation_angle_y_axis,
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_projection),
            false,
            &my_3d_matrices.projection,
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_normals_rotation),
            false,
            &my_3d_matrices.normals_rotation,
        );
        gl.uniform1f(Some(&self.u_opacity), 1.0);

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.y_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 1, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let y_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let y_location = y_vals.as_ptr() as u32 / 4;
        let y_array = js_sys::Float32Array::new(&y_memory_buffer)
            .subarray(y_location, y_location + y_vals.len() as u32);
        gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &y_array, WebGlRenderingContext::DYNAMIC_DRAW);

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.normals_buffer));
        gl.vertex_attrib_pointer_with_i32(2, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(2);

        let normals_vals = common_functions::get_grid_normals(super::super::constants::GRID_SIZE, &y_vals);
        let normals_vals_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let normals_vals_location = normals_vals.as_ptr() as u32 / 4;
        let normals_vals_array = js_sys::Float32Array::new(&normals_vals_memory_buffer).subarray(
            normals_vals_location,
            normals_vals_location + normals_vals.len() as u32,
        );
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.normals_buffer));
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &normals_vals_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );

        gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));

        gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, self.index_count, WebGlRenderingContext::UNSIGNED_SHORT, 0);
    }
}

fn create_grid_buffers(
    webgl_context: &WebGlRenderingContext,
) -> (web_sys::WebGlBuffer, web_sys::WebGlBuffer, i32) {
    let (vertex_array, index_array): (Vec<f32>, Vec<u16>) =
        common_functions::get_square_vertex_grid(10);
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let vertices_location = vertex_array.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
        vertices_location,
        vertices_location + vertex_array.len() as u32,
    );
    let buffer_position = webgl_context
        .create_buffer()
        .ok_or("failed to create buffer")
        .unwrap();
    webgl_context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer_position));
    webgl_context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    let indices_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let indices_location = index_array.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
        indices_location,
        indices_location + index_array.len() as u32,
    );
    let index_count = indices_array.length() as i32;
    let buffer_indices = webgl_context.create_buffer().unwrap();
    webgl_context.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&buffer_indices),
    );
    webgl_context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    return (buffer_position, buffer_indices, index_count);
}
