use std::{cell::RefCell, rc::Rc};

use cgmath::{vec3, Matrix4, Quaternion, SquareMatrix, Vector3};
use web_sys::WebGl2RenderingContext;
use weblog::console_log;

use super::{geometry::Geometry, graph::Node, material::Material, renderer::RenderingContext};

pub trait Renderable {
    fn render(&mut self, transform: &Transform, rendering_context: &RenderingContext);
}



pub struct Transform {
    pub position: Vector3<f32>,
    pub quaternion: Quaternion<f32>,
    pub matrix: Matrix4<f32>,
    pub matrix_world: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            quaternion: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            matrix: Matrix4::identity(),
            matrix_world: Matrix4::identity(),
        }
    }
}


pub struct Object3d {
    pub name: Option<String>,
    pub transform: Transform,
    pub renderer: Option<RefCell<Box<dyn Renderable>>>,
}

impl Object3d {
    pub fn new() -> Object3d {
        Object3d {
            name: None,
            transform: Transform::new(),
            renderer: None,
        }
    }

    pub fn update(&mut self, node: &Rc<Node<Object3d>>) {
        self.transform.matrix = Matrix4::from(self.transform.quaternion) * Matrix4::from_translation(self.transform.position);

        let parent = node.parent.borrow().upgrade();
        if parent.is_none() {
            self.transform.matrix_world = self.transform.matrix;
        } else {
            self.transform.matrix_world = parent.as_ref().unwrap().value.borrow().transform.matrix_world * self.transform.matrix;
        }
    }

}




pub struct Mesh<'a> {
    pub material: Material<'a>,
    pub geometry: Geometry,
}

impl<'a> Mesh<'a> {
    pub fn new(material: Material<'a>, geometry: Geometry) -> Mesh {
        Mesh {
            material: material,
            geometry: geometry,
        }
    }
}

impl Renderable for Mesh<'_> {
    fn render(&mut self, transform: &Transform, rendering_context: &RenderingContext) {
        let gl = rendering_context.gl;

        let light_direction = [-0.5, 0.5, 0.5];
        let ambient_color = [1.0, 1.0, 1.0, 1.0];

        let eye = vec3(
            rendering_context.camera_matrix.w.x,
            rendering_context.camera_matrix.w.y,
            rendering_context.camera_matrix.w.z,
        );
        // console_log!(format!("eye2={:?}", eye));

        let camera_matrix_invert = rendering_context.camera_matrix.invert().unwrap();
        let view_matrix: Matrix4<f32> = camera_matrix_invert;
        let model_view_matrix: Matrix4<f32> = camera_matrix_invert * transform.matrix_world;

        let mat = &mut self.material;
        let geo = &mut self.geometry;
        let vertices_count = geo.indexes.as_ref().unwrap().len();
        let program = mat.get_program(gl).unwrap();
        let vao = geo.get_vao(gl, program).unwrap();

        let uni_location = [
            gl.get_uniform_location(&program, "modelMatrix"),
            gl.get_uniform_location(&program, "modelViewMatrix"),
            gl.get_uniform_location(&program, "projectionMatrix"),
            gl.get_uniform_location(&program, "viewMatrix"),
            gl.get_uniform_location(&program, "cameraPosition"),
            gl.get_uniform_location(&program, "lightDirection"),
            gl.get_uniform_location(&program, "ambientColor"),
        ];

        gl.use_program(Some(program));
        gl.bind_vertex_array(Some(vao));

        gl.uniform_matrix4fv_with_f32_array(
            uni_location[0].as_ref(),
            false,
            transform.matrix_world.as_ref() as &[f32; 16],
        );
        gl.uniform_matrix4fv_with_f32_array(
            uni_location[1].as_ref(),
            false,
            model_view_matrix.as_ref() as &[f32; 16],
        );
        gl.uniform_matrix4fv_with_f32_array(
            uni_location[2].as_ref(),
            false,
            rendering_context.projection_matrix.as_ref() as &[f32; 16],
        );
        gl.uniform_matrix4fv_with_f32_array(uni_location[3].as_ref(), false, view_matrix.as_ref() as &[f32; 16]);
        gl.uniform3fv_with_f32_array(uni_location[4].as_ref(), &eye.as_ref() as &[f32; 3]);
        gl.uniform3fv_with_f32_array(uni_location[5].as_ref(), &light_direction);
        gl.uniform4fv_with_f32_array(uni_location[6].as_ref(), &ambient_color);
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            vertices_count as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );

        gl.bind_vertex_array(None);
    }
}
