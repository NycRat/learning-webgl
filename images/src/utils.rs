use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram};

pub fn get_window_size() -> (f32, f32) {
    let window = web_sys::window().unwrap();
    (
        window.inner_width().unwrap().as_f64().unwrap() as f32,
        window.inner_height().unwrap().as_f64().unwrap() as f32,
    )
}

pub fn create_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WebGlShader {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    return shader;
}

pub fn create_program(gl: &WebGl2RenderingContext, vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> WebGlProgram {
    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);
    return program;
}
