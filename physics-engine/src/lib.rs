use std::{cell::RefCell, rc::Rc};

use state::State;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub mod mouse;
pub mod objs;
pub mod state;
pub mod transformations;
pub mod utils;

const SPEED: f32 = 0.08;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;

    let size = 2000.0;
    let scale = window.device_pixel_ratio();

    canvas.set_width((size * scale) as u32);
    canvas.set_height((size * scale) as u32);

    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.enable(WebGl2RenderingContext::CULL_FACE);
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);

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

    gl.use_program(Some(&program));

    gl.uniform3f(uniform_reverse_light_direction_l.as_ref(), 0.5, 0.7, 1.0);

    gl.bind_vertex_array(vao.as_ref());

    gl.clear_color(0.2, 0.1, 0.2, 1.0);

    let state = Rc::new(RefCell::new(State::new()));

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let mut state = state.borrow_mut();
            state.pointer_locked = document.pointer_lock_element().is_some();
            web_sys::console::log_1(&state.pointer_locked.into());
        });
        window
            .add_event_listener_with_callback("pointerlockchange", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut state = state.borrow_mut();
            if state.pointer_locked {
                let (delta_x, delta_y) = (event.movement_x() as f32, event.movement_y() as f32);
                state.mouse.process_mouse_movement(delta_x, delta_y);
                // state.camera_rotation.0 -= delta_y / 1000.0;
                // state.camera_rotation.1 -= delta_x / 1000.0;
            }
        });
        window
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut state = state.borrow_mut();
            let key = event.key();
            state.keys_pressed.insert(key);
        });
        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut state = state.borrow_mut();
            let key = event.key();
            state.keys_pressed.remove(&key);
        });
        window
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            canvas.request_pointer_lock();
        });
        window
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let gl = gl.clone();
        let state = state.clone();
        let update_closure = Closure::<dyn FnMut()>::new(move || {
            let (size_x, size_y) = utils::get_window_size();

            let mut state = state.borrow_mut();

            if state.pointer_locked {
                let target = state.mouse.get_target();
                if state.keys_pressed.contains("a") {
                    state.camera_position[0] -= -target[2] * SPEED;
                    state.camera_position[2] -= target[0] * SPEED;
                }
                if state.keys_pressed.contains("d") {
                    state.camera_position[0] += -target[2] * SPEED;
                    state.camera_position[2] += target[0] * SPEED;
                }
                if state.keys_pressed.contains("w") {
                    for i in 0..3 {
                        state.camera_position[i] += target[i] * SPEED;
                    }
                }
                if state.keys_pressed.contains("s") {
                    for i in 0..3 {
                        state.camera_position[i] -= target[i] * SPEED;
                    }
                }
            }

            // web_sys::console::log_1(&format!("{state:?}").into());

            let projection_matrix = transformations::perspective(
                std::f32::consts::PI / (6.0),
                size_x / size_y,
                1.0,
                200.0,
            );

            let camera_matrix = utils::matrix_multiply(
                transformations::translation(
                    state.camera_position[0],
                    state.camera_position[1],
                    state.camera_position[2],
                ),
                transformations::look_at(
                    &state.camera_position,
                    &state.mouse.get_look_at_target(&state.camera_position),
                ),
            );

            let view_matrix = utils::invert_matrix(camera_matrix);
            let view_projection_matrix = utils::matrix_multiply(projection_matrix, view_matrix);

            let final_transformation_matrix = utils::matrix_multiply(
                utils::matrix_multiply(
                    view_projection_matrix,
                    transformations::translation(0.0, 0.0, -1.0),
                ),
                transformations::rotation_y(0.0),
            );

            gl.uniform_matrix4fv_with_f32_array(
                uniform_transformation_l.as_ref(),
                true,
                &final_transformation_matrix,
            );

            draw(&gl, vertices_len);
        });

        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                update_closure.as_ref().unchecked_ref(),
                1000 / 60,
            )
            .unwrap();
        update_closure.forget();
    }

    Ok(())
}

pub fn draw(gl: &WebGl2RenderingContext, vertices_len: i32) {
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertices_len);
}
