use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub mod objs;
pub mod transformations;
pub mod utils;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
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

    let cube_obj = objs::get_cube_obj();

    // THIS GOOD

    let vertex_shader = utils::create_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("./shader.vert"),
    );
    let fragment_shader = utils::create_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("./shader.frag"),
    );
    let program = utils::create_program(&gl, &vertex_shader, &fragment_shader);

    let attrib_position_l = gl.get_attrib_location(&program, "a_pos") as u32;
    let attrib_normal_l = gl.get_attrib_location(&program, "a_normal") as u32;
    // let attrib_color_l = gl.get_attrib_location(&program, "a_color") as u32;

    let uniform_transformation_l = gl.get_uniform_location(&program, "u_transformation");
    let uniform_reverse_light_direction_l =
        gl.get_uniform_location(&program, "u_reverse_light_direction");

    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());

    //{
    // GET DATA INTO BUFFER
    let vertices_buffer = gl.create_buffer();
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        vertices_buffer.as_ref(),
    );

    let vertices_len = objs::set_positions(&gl, &cube_obj);

    gl.enable_vertex_attrib_array(attrib_position_l);
    gl.vertex_attrib_pointer_with_i32(
        attrib_position_l,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    // GET DATA INTO BUFFER
    let normal_buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
    objs::set_normals(&gl, &cube_obj);
    gl.enable_vertex_attrib_array(attrib_normal_l);
    gl.vertex_attrib_pointer_with_i32(
        attrib_normal_l,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    // }

    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    gl.enable(WebGl2RenderingContext::CULL_FACE);

    gl.clear_color(0.2, 0.1, 0.2, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.use_program(Some(&program));

    gl.uniform3f(uniform_reverse_light_direction_l.as_ref(), 0.5, 0.7, 1.0);

    gl.bind_vertex_array(vao.as_ref());

    let mut x = 0.0;
    let state = Rc::new(Cell::new(1000.0f32));

    let mut now = web_sys::js_sys::Date::now();

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::WheelEvent| {
            let val = state.get();
            state.set((val + event.delta_y() as f32).clamp(0.0, 2000.0));
        });
        window
            .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    {
        let gl = gl.clone();
        let state = state.clone();

        // let update_closure =
        let window = window.clone();
        *g.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            let (size_x, size_y) = utils::get_window_size();
            x += 1.0;

            // web_sys::console::log_1(&state.get().into());

            let projection_matrix = transformations::perspective(
                std::f32::consts::PI / (8.0),
                size_x / size_y,
                1.0,
                200.0,
            );

            let camera_matrix =
                transformations::translation(0.0, 0.0, 2.0 + state.get() / 2000.0 * 26.0);
            //     utils::matrix_multiply(
            //     utils::matrix_multiply(
            //         transformations::rotation_y(-x / size_x * 1.0),
            //         transformations::rotation_x(-y / size_y * 1.0),
            //     ),
            //     transformations::translation(0.0, 0.0, 4.0),
            // );
            let view_matrix = utils::invert_matrix(camera_matrix);
            let view_projection_matrix = utils::matrix_multiply(projection_matrix, view_matrix);

            let final_transformation_matrix = utils::matrix_multiply(
                utils::matrix_multiply(
                    view_projection_matrix,
                    transformations::translation(0.0, 0.0, -3.5),
                ),
                transformations::rotation_y(x / 100.0),
                // transformations::rotation_y(0.0),
            );

            gl.uniform_matrix4fv_with_f32_array(
                uniform_transformation_l.as_ref(),
                true,
                &final_transformation_matrix,
            );

            draw(&gl, vertices_len);

            let now2 = web_sys::js_sys::Date::now();
            web_sys::console::log_1(&(now2 - now).into());
            now = now2;

            window
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }));

        // window.add_event_listener_with_callback("mousemove", update_closure.as_ref().unchecked_ref())?;

        // window
        //     .set_interval_with_callback_and_timeout_and_arguments_0(
        //         update_closure.as_ref().unchecked_ref(),
        //         // 1000 / 30,
        //         1,
        //     )
        //     .unwrap();

        // update_closure.forget();
    }

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();

    Ok(())
}

pub fn draw(gl: &WebGl2RenderingContext, vertices_len: i32) {
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertices_len);
}
