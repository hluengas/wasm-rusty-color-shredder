use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlShader;

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
