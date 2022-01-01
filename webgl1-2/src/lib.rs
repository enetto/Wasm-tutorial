use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::WebGl2RenderingContext as GL;
mod webgl;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame'");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let height: f32 = 500.;
    let width: f32 = 500.;

    let gl = webgl::get_webgl2_context(height as u32, width as u32).unwrap();

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
        -1.0,  0.0, 0.0, 
         1.0,  0.0, 0.0, 
         0.0,  1.0, 0.0
    ];

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

    //animation frame
    let frame = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = frame.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move ||{

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.);
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    gl.bind_vertex_array(Some(&triangle_vao));
    gl.draw_elements_with_i32(GL::TRIANGLES, index.len() as i32, GL::UNSIGNED_SHORT, 0);

    gl.flush();

    request_animation_frame(frame.borrow().as_ref().unwrap());

    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
    
}