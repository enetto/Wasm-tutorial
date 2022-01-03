use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::WebGl2RenderingContext as GL;
mod webgl;
mod matrix;
mod cube;

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

    let uni_location = [
        gl.get_uniform_location(&program, "pvmMatrix").unwrap(),
        gl.get_uniform_location(&program, "invMatrix").unwrap(),
        gl.get_uniform_location(&program, "LightDirection").unwrap(),
        gl.get_uniform_location(&program, "EyeDirection").unwrap(),
        gl.get_uniform_location(&program, "AmbientColor").unwrap(),
    ];

    let att_location = [0, 1, 2];

    let att_stride = [3, 4, 3];

    //正方形
    let white_cube = cube::create_cube(2., 1., 1., 1., 1.);

    
    let pos = white_cube.0;
    let cor = white_cube.1;
    let nor = white_cube.2;
    let idx = white_cube.3;
    

    //set vao
    let white_cube_vao = webgl::create_vao(
        &gl,
        &[&pos, &cor, &nor],
        None,
        &att_location,
        &att_stride,
        &idx,
    );

    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);
    gl.depth_func(GL::LEQUAL);

    let mut p_matrix = matrix::Matrix::new();
    let mut v_matrix = matrix::Matrix::new();
    let mut m_matrix = matrix::Matrix::new();
    let mut pvm_matrix = matrix::Matrix::new();
    let mut tmp_matrix = matrix::Matrix::new();
    let mut inv_matrix = matrix::Matrix::new();

    let EyeDirection = [0., 0., 20.];

    let LightDirection = [-0.5, 0.5, 0.5];
    let AmbientColor = [0.1, 0.1, 0.1, 0.1];

    v_matrix.look_at(&EyeDirection, &[0., 0., 0.], &[0., 1., 0.]);
    p_matrix.perspective(width / height, 45., 0.1, 100.);
    tmp_matrix.substitution(&p_matrix).multiply(&v_matrix);

    //animation frame
    let frame = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = frame.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move ||{

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.);
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    m_matrix.set_identity();
    pvm_matrix.substitution(&tmp_matrix).multiply(&m_matrix);
    inv_matrix.substitution(&m_matrix).inverse().unwrap();

    gl.bind_vertex_array(Some(&white_cube_vao));

    gl.uniform_matrix4fv_with_f32_array(Some(&uni_location[0]), false, &pvm_matrix.get_value());
    gl.uniform_matrix4fv_with_f32_array(Some(&uni_location[1]), false, &inv_matrix.get_value());
    gl.uniform3fv_with_f32_array(Some(&uni_location[2]), &LightDirection);
    gl.uniform3fv_with_f32_array(Some(&uni_location[3]), &EyeDirection);
    gl.uniform4fv_with_f32_array(Some(&uni_location[4]), &AmbientColor);
    gl.draw_elements_with_i32(GL::TRIANGLES, idx.len() as i32, GL::UNSIGNED_SHORT, 0);

    gl.flush();

    request_animation_frame(frame.borrow().as_ref().unwrap());

    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
    
}