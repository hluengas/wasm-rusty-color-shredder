#[allow(dead_code)]
pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec2 aTexCoord;

    uniform mat4 uTransform;
    
    void main() {
        // Multiply the position by the matrix.
        gl_Position = uTransform * aPosition;
        
        // Pass the texcoord to the fragment shader.
        v_texcoord = aTexCoord;
    }
"#;