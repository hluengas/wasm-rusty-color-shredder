use web_sys::WebGlRenderingContext;
use web_sys::WebGlShader;
use web_sys::WebGlProgram;

fn compile_shader(
    webgl_context: &WebGlRenderingContext,
    shader_type: u32,
    shader_source: &str,
) -> Result<WebGlShader, String> {
    let shader = webgl_context.create_shader(shader_type).ok_or_else(|| String::from("error creating shader"))?;
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

fn link_program(
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
        return Err(webgl_context.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("error attaching shaders and linking program")));
    }
}
