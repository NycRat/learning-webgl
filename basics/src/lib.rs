use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

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

    context.viewport(0, 0, thingy.width() as i32, thingy.height() as i32);

    // vert shader assigns gl position
    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        precision highp float;

        // uniform mat4 u_Transform;

        in vec4 position;
        in vec4 idkColor;

        out vec4 inColor;

        void main() {
            inColor = idkColor;
            // inColor = vec4(position.x, 0.0, 0.0, 1);
            // gl_Position = u_Transform * position;
            gl_Position = position;
        }
        "##,
    )?;

    // frag shader assigns rgba value
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;

        in vec4 inColor;
        out vec4 outColor;
        
        void main() {
            // outColor = vec4(0.5, 0.2, 1, 1);
            outColor = inColor;
        }
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    // let vertices: [f32; 12] = [
    //     -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, -0.3, -0.3, 0.0, -1.0, 1.0, 0.0, // stuff
    // ];

    // let thingy = [
    //     [0.0, -0.25, -0.50],
    //     [0.0, 0.25, 0.00],
    //     [0.5, -0.25, 0.25],
    //     [-0.5, -0.25, 0.25],
    // ];

    let vertices: [f32; 36] = [
        0.5, -0.25, 0.25, 0.0, 0.25, 0.0, -0.5, -0.25, 0.25, -0.5, -0.25, 0.25, 0.0, 0.25, 0.0,
        0.0, -0.25, -0.5, 0.0, -0.25, -0.5, 0.0, 0.25, 0.0, 0.5, -0.25, 0.25, 0.0, -0.25, -0.5,
        0.5, -0.25, 0.25, -0.5, -0.25, 0.25,
    ];
    let colors: [f32; 48] = [
        1.0, 0.5, 0.6, 1.0, // color
        1.0, 0.5, 0.6, 1.0, // color
        1.0, 0.5, 0.6, 1.0, // color
        0.0, 0.5, 1.0, 1.0, // color
        0.0, 0.5, 1.0, 1.0, // color
        0.0, 0.5, 1.0, 1.0, // color
        0.0, 0.5, 0.6, 1.0, // color
        0.0, 0.5, 0.6, 1.0, // color
        0.0, 0.5, 0.6, 1.0, // color
        0.2, 0.5, 0.6, 1.0, // color
        0.2, 0.5, 0.6, 1.0, // color
        0.2, 0.5, 0.6, 1.0, // color
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let color_attribute_location = context.get_attrib_location(&program, "idkColor");

    // web_sys::console::log_1(&"wha".into());
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
            // WebGl2RenderingContext::DYNAMIC_DRAW,
        );
    }
    //
    // let aaa = context
    //     .create_vertex_array()
    //     .ok_or("Could not create vertex array object")?;
    // context.bind_vertex_array(Some(&aaa));
    //
    context.vertex_attrib_pointer_with_i32(
        color_attribute_location as u32,
        4,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(color_attribute_location as u32);
    //
    // context.bind_vertex_array(Some(&aaa));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    // context.clear_color(1.0, 1.0, 1.0, 1.0);
    // context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    // context.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, vert_count);
    // context.draw_arrays(WebGl2RenderingContext::LINE_LOOP, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
