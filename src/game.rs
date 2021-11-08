use std::{cell::RefCell, rc::Rc};

use cgmath::{vec3, Deg, Matrix4, Point3, Quaternion, Rotation3, Transform};
use js_sys::Date;
use wasm_bindgen::JsValue;
use weblog::{console_error, console_log};

use crate::core::{
    geometry::Geometry,
    material::Material,
    object3d::{Mesh, Node},
    renderer::Renderer,
    scene::Scene,
    shapes,
};

pub fn example1() -> Result<(Box<dyn FnMut(f64, f64) -> ()>, Box<dyn FnMut() -> ()>), JsValue> {
    console_log!("Starting example 1");

    // Create cam

    // Create scene
    let mut scene = Scene::new();

    // Add elements
    let cube0 = create_cube(2.5, String::from("cube0"));
    scene.add_child(cube0.clone());

    let cube1 = create_cube(1.5, String::from("cube1"));
    cube1.borrow_mut().transform.position = vec3(5.0, 0.0, 0.0);
    cube0.borrow_mut().add_child(cube0.clone(), cube1.clone());

    // Create renderer
    let mut renderer = Renderer::new("canvas");
    renderer.create()?;
    let renderer_rc = Rc::new(RefCell::new(renderer));

    let renderer = renderer_rc.clone();
    let resize_fn = move |width: f64, height: f64| {
        renderer.borrow_mut().set_size(width, height);
    };

    let mut total_time: f32 = 0.0;
    let mut time = Date::now();
    let renderer = renderer_rc.clone();

    let camera = Matrix4::from_translation(vec3(0.0, 7.5, 15.0)) * Matrix4::from_angle_x(Deg(-25.0));

    let update_fn = move || {
        let newtime = Date::now();
        let ellapsed = ((newtime - time) / 1000.0) as f32;
        total_time = total_time + ellapsed;
        time = newtime;

        //rotate mesh
        {
            let mut m = cube0.borrow_mut();
            m.transform.quaternion = Quaternion::from_angle_y(Deg(ellapsed * 90.0)) * m.transform.quaternion;
        }

        // Do rendering
        let res = renderer.borrow_mut().draw(&mut scene, &camera, ellapsed);
        if res.is_err() {
            console_error!(res.unwrap_err());
        }
    };

    Ok((Box::new(resize_fn), Box::new(update_fn)))
}

fn create_cube(size: f32, name: String) -> Rc<RefCell<Node>> {
    let cube = shapes::cube(size, size, size);

    let mut geometry = Geometry::new();
    geometry.set_indexes(&cube.0);
    geometry.set_positions(&cube.1);
    geometry.set_normals(&cube.2);

    let mut material = Material::new();
    material.vertex_shader = include_str!("core/shaders/vertex.glsl");
    material.fragment_shader = include_str!("core/shaders/fragment.glsl");

    let mesh0 = Rc::new(RefCell::new(Mesh::new(material, geometry)));
    let mesh = Rc::new(RefCell::new(Node::new()));
    mesh.borrow_mut().renderer.replace(mesh0);
    mesh.borrow_mut().name = Some(name);

    mesh
}
