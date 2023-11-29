use crate::webgl::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub mod transformations;
pub mod webgl;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let size = 2000.0;

    let thingy: HtmlCanvasElement = context.canvas().unwrap().dyn_into().unwrap();

    let scale = window.device_pixel_ratio();
    canvas.set_width((size * scale) as u32);
    canvas.set_height((size * scale) as u32);

    // let canvas_size = (canvas.width(), canvas.height());
    let canvas_size = (966 as f32, 966 as f32);

    context.viewport(0, 0, thingy.width() as i32, thingy.height() as i32);

    // vert shader assigns gl position
    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("./shader.vert"),
    )?;

    // frag shader assigns rgba value
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("./shader.frag"),
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 36] = [
        0.5, -0.25, 0.25, 0.0, 0.25, 0.0, -0.5, -0.25, 0.25, -0.5, -0.25, 0.25, 0.0, 0.25, 0.0,
        0.0, -0.25, -0.5, 0.0, -0.25, -0.5, 0.0, 0.25, 0.0, 0.5, -0.25, 0.25, 0.0, -0.25, -0.5,
        0.5, -0.25, 0.25, -0.5, -0.25, 0.25,
    ];
    let colors: [f32; 48] = [
        1.0, 0.0, 0.0, 1.0, // color
        1.0, 0.0, 0.0, 1.0, // color
        1.0, 0.0, 0.0, 1.0, // color
        0.0, 1.0, 0.0, 1.0, // color
        0.0, 1.0, 0.0, 1.0, // color
        0.0, 1.0, 0.0, 1.0, // color
        0.0, 0.0, 1.0, 1.0, // color
        0.0, 0.0, 1.0, 1.0, // color
        0.0, 0.0, 1.0, 1.0, // color
        0.4, 0.8, 0.0, 1.0, // color
        0.4, 0.8, 0.0, 1.0, // color
        0.4, 0.8, 0.0, 1.0, // color
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let color_attribute_location = context.get_attrib_location(&program, "idkColor");

    let offset_attribute_location = context.get_uniform_location(&program, "u_offset");
    let matrix_attribute_location = context.get_uniform_location(&program, "u_matrix");
    let matrix2_attribute_location = context.get_uniform_location(&program, "u_matrix2");

    context.uniform4f(offset_attribute_location.as_ref(), 0.0, 0.0, 0.0, 0.0);

    context.uniform_matrix4fv_with_f32_array(
        matrix_attribute_location.as_ref(),
        true,
        &[
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );

    context.uniform_matrix4fv_with_f32_array(
        matrix2_attribute_location.as_ref(),
        true,
        &[
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );

    web_sys::console::log_1(&position_attribute_location.into());
    web_sys::console::log_1(&color_attribute_location.into());

    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
            // WebGl2RenderingContext::DYNAMIC_DRAW,
        );
    }

    // let vao = context
    //     .create_vertex_array()
    //     .ok_or("Could not create vertex array object")?;
    // context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);
    // context.bind_vertex_array(Some(&vao));

    let buffer2 = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer2));
    unsafe {
        let colors_array_buf_view = js_sys::Float32Array::view(&colors);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &colors_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    context.vertex_attrib_pointer_with_i32(
        color_attribute_location as u32,
        4,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(color_attribute_location as u32);

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);

    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let (x, y) = (event.offset_x() as f32, event.offset_y() as f32);
            let new_offset = (
                x / canvas_size.0 * 2.0 - 1.0,
                -(y / canvas_size.1 * 2.0 - 1.0),
            );
            context.uniform_matrix4fv_with_f32_array(
                matrix_attribute_location.as_ref(),
                true,
                &transformations::rotation_y(new_offset.0 * 2.0 * std::f32::consts::PI),
            );
            context.uniform_matrix4fv_with_f32_array(
                matrix2_attribute_location.as_ref(),
                true,
                &transformations::rotation_x(new_offset.1 * 2.0 * std::f32::consts::PI),
            );

            draw(&context, vert_count);
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
