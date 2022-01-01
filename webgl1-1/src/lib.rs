use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;
mod webgl;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let gl = webgl::get_webgl2_context().unwrap();

    let program = webgl::create_program(
        &gl,
        include_str!("shader/vertex.vert"),
        include_str!("shader/fragment.frag"),
    )
    .unwrap();

    gl.use_program(Some(&program));

    let att_location = [0,1];

    let att_stride: [i32; 2] = [3, 4];

    let position: Vec<f32> = vec![
        -0.7, -0.7, 0.0, 
         0.7, -0.7, 0.0, 
         0.0,  0.7, 0.0];

    let color: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0
    ];

    let index: Vec<u16> = vec![0, 1, 2];

    //set vao
    let triangle_vao = webgl::create_vao(
        &gl, 
        &[&position, &color], 
        None, 
        &att_location, 
        &att_stride, 
        &index,
    );

    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);
    gl.depth_func(GL::LEQUAL);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.);
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    gl.bind_vertex_array(Some(&triangle_vao));
    gl.draw_elements_with_i32(GL::TRIANGLES, index.len() as i32, GL::UNSIGNED_SHORT, 0);

    gl.flush();

    Ok(())
}