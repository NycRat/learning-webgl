use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement, HtmlImageElement};

pub mod transformations;
pub mod utils;
pub mod objs;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let image = web_sys::HtmlImageElement::new().unwrap();
    {
        let image = image.clone();
        let image2 = image.clone();
        image.set_src("rose.jpg");
        let closure = Closure::<dyn FnMut()>::new(move || {
            actual(&image2).unwrap();
        });
        image.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    Ok(())
}

fn actual(image: &HtmlImageElement) -> Result<(), JsValue> {
    // TODO DO ACTUAL GOOD WIDTH AND HEIGTH THING

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let size = 2000.0;
    let scale = window.device_pixel_ratio();

    canvas.set_width((size * scale) as u32);
    canvas.set_height((size * scale) as u32);

    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // TODO END

    // THIS GOOD

    let vertex_shader = utils::create_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, include_str!("./shader.vert"));
    let fragment_shader = utils::create_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, include_str!("./shader.frag"));
    let program = utils::create_program(&gl, &vertex_shader, &fragment_shader);

    let attrib_position_l = gl.get_attrib_location(&program, "a_pos") as u32;
    let attrib_color_l = gl.get_attrib_location(&program, "a_color") as u32;
    let attrib_tex_coord_l = gl.get_attrib_location(&program, "a_tex_coord") as u32;

    let uniform_resolution_l = gl.get_uniform_location(&program, "u_resolution");

    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());

    //{
    // GET DATA INTO BUFFER
    let vertices_buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vertices_buffer.as_ref());
    objs::set_vertices(&gl);
    gl.enable_vertex_attrib_array(attrib_position_l);
    gl.vertex_attrib_pointer_with_i32(attrib_position_l, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

    // GET DATA INTO BUFFER
    // let color_buffer = gl.create_buffer();
    // gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, color_buffer.as_ref());
    // objs::set_colors(&gl);
    // gl.enable_vertex_attrib_array(attrib_color_l);
    // gl.vertex_attrib_pointer_with_i32(attrib_color_l, 3, WebGl2RenderingContext::UNSIGNED_BYTE, true, 0, 0);

    // GET DATA INTO BUFFER
    let tex_coord_buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, tex_coord_buffer.as_ref());
    objs::set_tex_coords(&gl);
    gl.enable_vertex_attrib_array(attrib_tex_coord_l);
    gl.vertex_attrib_pointer_with_i32(attrib_tex_coord_l, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    //}

    //{
    // TEST THING BTW
    // let tex = objs::test(&gl);
    let uniform_texture_l = gl.get_uniform_location(&program, "u_texture");
    // let unit = 5;
    // gl.active_texture(WebGl2RenderingContext::TEXTURE0 + unit);
    // gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, tex.as_ref());

    let texture = gl.create_texture();
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + 0);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

    let mipLevel = 0;
    let internalFormat = WebGl2RenderingContext::RGBA;
    let srcFormat = WebGl2RenderingContext::RGBA;
    let srcType = WebGl2RenderingContext::UNSIGNED_BYTE;
    gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                   WebGl2RenderingContext::TEXTURE_2D,
                  mipLevel,
                  internalFormat as i32,
                  srcFormat,
                  srcType,
                  &image
        ).unwrap();
    // }
    // HERE IS WHERE YOU LEFT OFF

    gl.use_program(Some(&program));

    // gl.uniform1i(uniform_sampler_l.as_ref(), unit as i32);

    let (x,y) = (image.width() as f32, image.height() as f32);

    // web_sys::console::log_2(&x.into(), &y.into());

    gl.uniform2f(uniform_resolution_l.as_ref(), 1044.0, 966.0);
    // gl.uniform2f(uniform_resolution_l.as_ref(), x, y);
    // gl.uniform4f(uniform_color_l.as_ref(), 1.0, 1.0, 0.8, 1.0);
    gl.uniform1i(uniform_texture_l.as_ref(), 0);

    gl.bind_vertex_array(vao.as_ref());

    // DRAW
    gl.clear_color(0.2, 0.1, 0.2, 1.0);

    let vertices_len = 12;
    draw(&gl, vertices_len);

    Ok(())
}

pub fn draw(gl: &WebGl2RenderingContext, vertices_len: i32) {
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertices_len / 2);
}
