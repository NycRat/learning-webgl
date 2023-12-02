use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub fn set_vertices(gl: &WebGl2RenderingContext) {
    unsafe {
        let vertices = 
        [
            0.0, 0.0,
            800.0, 0.0,
            0.0, 800.0,
            0.0, 800.0,
            800.0, 0.0,
            800.0, 800.0,
        ];

        let array_buf_view = js_sys::Float32Array::view(&vertices);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn set_colors(gl: &WebGl2RenderingContext) {
    unsafe {
        let colors = 
        [
            0, 0, 120,
            0, 20, 0,
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ];

        let array_buf_view = js_sys::Uint8Array::view(&colors);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn set_tex_coords(gl: &WebGl2RenderingContext) {
    unsafe {
        let tex_coords = 
        [
            0.0,  0.0,
            1.0,  0.0,
            0.0,  1.0,

            0.0,  1.0,
            1.0,  0.0,
            1.0,  1.0
        ];

        let array_buf_view = js_sys::Float32Array::view(&tex_coords);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn test(gl: &WebGl2RenderingContext) -> Option<WebGlTexture> {
    let tex = gl.create_texture();
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, tex.as_ref());
    let level = 0;
    let internal_format = WebGl2RenderingContext::RGBA as i32;
    let width = 2;
    let height = 1;
    let border = 0; // MUST ALWAYS BE ZERO
    let format = WebGl2RenderingContext::RGBA;
    let type2 = WebGl2RenderingContext::UNSIGNED_BYTE;
    // let data = js_sys::Uint8Array::view(&[255, 0, 0, 255, 0, 255, 0, 255]);
    let data = Some([255, 0, 0, 255, 0, 255, 0, 255].as_slice());
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(WebGl2RenderingContext::TEXTURE_2D,
                  level,
                  internal_format,
                  width,
                  height,
                  border,
                  format,
                  type2,
                  data).unwrap();
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
    return tex;
}
