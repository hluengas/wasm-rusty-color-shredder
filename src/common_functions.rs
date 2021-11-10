use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlShader;
use nalgebra::{Matrix4,Perspective3};
use super::constants::*;


pub fn link_program(
    webgl_context: &WebGlRenderingContext,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<WebGlProgram, String> {
    let program = webgl_context
        .create_program()
        .ok_or_else(|| String::from("error creating program"))?;

    let vertex_shader = compile_shader(
        &webgl_context,
        WebGlRenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )
    .unwrap();

    let fragment_shader = compile_shader(
        &webgl_context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )
    .unwrap();

    webgl_context.attach_shader(&program, &vertex_shader);
    webgl_context.attach_shader(&program, &fragment_shader);
    webgl_context.link_program(&program);

    if webgl_context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(program);
    } else {
        return Err(webgl_context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("error attaching shaders and linking program")));
    }
}

fn compile_shader(
    webgl_context: &WebGlRenderingContext,
    shader_type: u32,
    shader_source: &str,
) -> Result<WebGlShader, String> {
    let shader = webgl_context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("error creating shader"))?;
    webgl_context.shader_source(&shader, shader_source);
    webgl_context.compile_shader(&shader);

    if webgl_context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(shader);
    } else {
        return Err(webgl_context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("unable to get context log info")));
    }
}

pub fn translation_matrix(x_translate: f32, y_translate: f32, z_translate: f32) -> [f32; 16] {
    let mut return_var = [0.0; 16];

    return_var[0] = 1.0;
    return_var[5] = 1.0;
    return_var[10] = 1.0;
    return_var[15] = 1.0;

    return_var[12] = x_translate;
    return_var[13] = y_translate;
    return_var[14] = z_translate;

    return return_var;
}

pub fn scaling_matrix(x_scale: f32, y_scale: f32, z_scale: f32) -> [f32; 16] {
    let mut return_var = [0.0; 16];

    return_var[0] = x_scale;
    return_var[5] = y_scale;
    return_var[10] = z_scale;
    return_var[15] = 1.0;

    return return_var;
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut product_matrix = [0.; 16];

    product_matrix[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    product_matrix[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    product_matrix[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    product_matrix[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    product_matrix[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    product_matrix[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    product_matrix[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    product_matrix[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    product_matrix[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    product_matrix[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    product_matrix[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    product_matrix[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    product_matrix[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    product_matrix[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    product_matrix[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    product_matrix[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return product_matrix;
}

pub fn get_square_vertex_grid(grid_space_dimension: usize) -> (Vec<f32>, Vec<u16>) {
    let grid_point_dimension = grid_space_dimension + 1;

    let mut vertices: Vec<f32> = vec![0.0; 3 * grid_point_dimension * grid_point_dimension];
    let mut indices: Vec<u16> = vec![0; 6 * grid_space_dimension * grid_space_dimension];

    let graph_layout_width: f32 = 2.0;
    let grid_unit_size: f32 = graph_layout_width / grid_space_dimension as f32;

    for z in 0..grid_point_dimension {
        for x in 0..grid_point_dimension {
            // get the linear index into vertex array from the grid coords
            let index = 3 * (z * grid_point_dimension + x);

            // calculate x and z vertices, y = 0
            vertices[index + 0] = -1.0 + (x as f32) * grid_unit_size;
            vertices[index + 1] = 0.0;
            vertices[index + 2] = -1.0 + (z as f32) * grid_unit_size;

            if z < grid_space_dimension && x < grid_space_dimension {
                // get the linear index into index array from the grid coords
                let index = 6 * (z * grid_space_dimension + x);

                // calculate grid indices
                let vertex_index_top_left = (z * grid_point_dimension + x) as u16;
                let vertex_index_bottom_left = vertex_index_top_left + grid_point_dimension as u16;
                let vertex_index_top_right = vertex_index_top_left + 1;
                let vertex_index_bottom_right = vertex_index_bottom_left + 1;

                // first triangle (bottom)
                indices[index + 0] = vertex_index_top_left;
                indices[index + 1] = vertex_index_bottom_left;
                indices[index + 2] = vertex_index_bottom_right;
                
                // second triangle (top)
                indices[index + 3] = vertex_index_top_left;
                indices[index + 4] = vertex_index_bottom_right;
                indices[index + 5] = vertex_index_top_right;
            }
        }
    }

    return (vertices, indices);
}

pub struct Matrices3D {
    pub normals_rotation: [f32; 16],
    pub projection: [f32; 16],
}

pub fn get_3d_matrices(
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
    rotation_angle_x_axis: f32,
    rotation_angle_y_axis: f32,
) -> Matrices3D {
    let mut return_var = Matrices3D {
        normals_rotation: [0.; 16],
        projection: [0.; 16],
    };

    let rotate_x_axis: [f32; 16] = [
        1., 0.,                          0.,                            0.,
        0., rotation_angle_x_axis.cos(), -rotation_angle_x_axis.sin(),  0.,
        0., rotation_angle_x_axis.sin(), rotation_angle_x_axis.cos(),   0.,
        0., 0.,                          0.,                            1.,
    ];

    let rotate_y_axis: [f32; 16] = [
        rotation_angle_y_axis.cos(),  0., rotation_angle_y_axis.sin(), 0.,
        0.,                           1., 0.,                          0.,
        -rotation_angle_y_axis.sin(), 0., rotation_angle_y_axis.cos(), 0.,
        0.,                           0., 0.,                          1.,
    ];

    let rotation_matrix = mult_matrix_4(rotate_x_axis, rotate_y_axis);

    let aspect: f32 = canvas_width / canvas_height;
    let scale_x = (right - left) / canvas_width;
    let scale_y = (top - bottom) / canvas_height;
    let scale = scale_y;

    let translation_matrix: [f32; 16] = translation_matrix(
        -1. + scale_x + 2. * left / canvas_width,
        -1. + scale_y + 2. * bottom / canvas_height,
        Z_PLANE,
    );

    let scale_matrix: [f32; 16] = scaling_matrix(scale, scale, 0.);
    let rotation_scale = mult_matrix_4(rotation_matrix, scale_matrix);
    let combined_transform = mult_matrix_4(rotation_scale, translation_matrix);
    let perspective_matrix_tmp: Perspective3<f32> = Perspective3::new(aspect, FIELD_OF_VIEW, Z_NEAR, Z_FAR);
    let mut perspective: [f32; 16] = [0.; 16];
    perspective.copy_from_slice(perspective_matrix_tmp.as_matrix().as_slice());

    return_var.projection = mult_matrix_4(combined_transform, perspective);

    let normal_matrix = Matrix4::new(
        rotation_matrix[0],
        rotation_matrix[1],
        rotation_matrix[2],
        rotation_matrix[3],
        rotation_matrix[4],
        rotation_matrix[5],
        rotation_matrix[6],
        rotation_matrix[7],
        rotation_matrix[8],
        rotation_matrix[9],
        rotation_matrix[10],
        rotation_matrix[11],
        rotation_matrix[12],
        rotation_matrix[13],
        rotation_matrix[14],
        rotation_matrix[15],
    );

    match normal_matrix.try_inverse() {
        Some(inv) => {
            return_var.normals_rotation.copy_from_slice(inv.as_slice());
        }
        None => {}
    }

    return_var
}