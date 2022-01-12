use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub fn get_webgl2_context(height: u32, width: u32) -> Result<GL, String> {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| String::from("canvas doesn't exist :("))?;
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas.set_height(height);
    canvas.set_width(width);

    let gl: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .ok_or_else(|| String::from("webgl is not supported in this browser :("))?
        .dyn_into()
        .unwrap();

    Ok(gl)
}

pub fn create_program(
    gl: &GL,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
    gl: &GL, 
    shader_type: u32, 
    source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}


pub fn create_vao(
    gl: &GL,
    vbo_data: &[&[f32]],
    vbo_divisor_data: Option<&[Option<u32>]>,
    att_location: &[u32],
    att_stride: &[i32],
    ibo_data: &[u16],
) -> WebGlVertexArrayObject {
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(&vao));

    for i in 0..vbo_data.len() {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        unsafe {
            let f32_array = js_sys::Float32Array::view(vbo_data[i]);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::STATIC_DRAW)
        }
        gl.enable_vertex_attrib_array(att_location[i]);
        gl.vertex_attrib_pointer_with_i32(att_location[i], att_stride[i], GL::FLOAT, false, 0, 0);

        match vbo_divisor_data {
            Some(divisors) => match divisors[i] {
                Some(divisor) => gl.vertex_attrib_divisor(att_location[i], divisor),
                None => (),
            },
            None => (),
        }
    }

    let ibo = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
    unsafe {
        let ui16_array = js_sys::Uint16Array::view(ibo_data);
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &ui16_array,
            GL::STATIC_DRAW,
        );
    }
    gl.bind_vertex_array(None);
    vao
}

#[allow(dead_code)]
pub struct FramebufferObject {
    pub frame: WebGlFramebuffer,
    pub depth: WebGlRenderbuffer,
    pub tex: WebGlTexture,
}

#[allow(dead_code)]
pub fn create_framebuffer(gl: &GL, width: i32, height: i32) -> Result<FramebufferObject, String> {
    let frame_buffer = gl.create_framebuffer().unwrap();

    gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&frame_buffer));

    let depth_render_buffer = gl.create_renderbuffer().unwrap();
    gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&depth_render_buffer));

    gl.renderbuffer_storage(GL::RENDERBUFFER, GL::DEPTH_COMPONENT16, width, height);

    gl.framebuffer_renderbuffer(
        GL::FRAMEBUFFER,
        GL::DEPTH_ATTACHMENT,
        GL::RENDERBUFFER,
        Some(&depth_render_buffer),
    );

    let f_texture = gl.create_texture().unwrap();
    gl.bind_texture(GL::TEXTURE_2D, Some(&f_texture));

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        GL::TEXTURE_2D,
        0,
        GL::RGBA as i32,
        width,
        height,
        0,
        GL::RGBA,
        GL::UNSIGNED_BYTE,
        None,
    )
    .unwrap();

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

    gl.framebuffer_texture_2d(
        GL::FRAMEBUFFER,
        GL::COLOR_ATTACHMENT0,
        GL::TEXTURE_2D,
        Some(&f_texture),
        0,
    );

    gl.bind_texture(GL::TEXTURE_2D, None);
    gl.bind_renderbuffer(GL::RENDERBUFFER, None);
    gl.bind_framebuffer(GL::FRAMEBUFFER, None);

    Ok(FramebufferObject {
        frame: frame_buffer,
        depth: depth_render_buffer,
        tex: f_texture,
    })
}