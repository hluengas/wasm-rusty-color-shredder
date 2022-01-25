#[allow(dead_code)]
pub const SHADER: &str = r#"
    precision mediump float;

    // Passed in from the vertex shader.
    varying vec2 v_texcoord;
    
    uniform float uOpacity;

    void main() {
        gl_FragColor = texture2D(u_texture, v_texcoord) * uOpacity;
    }
"#;