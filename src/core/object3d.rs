use cgmath::{vec3, Matrix4, Quaternion, SquareMatrix, Vector3};
use web_sys::WebGl2RenderingContext;
use weblog::console_log;

use super::{geometry::Geometry, material::Material, renderer::RenderingContext};

pub trait Updatable {
    fn update(&mut self);
}
pub trait Renderable {
    fn render(&mut self, rendering_context: &RenderingContext);
}

pub trait GraphNode: Updatable + Renderable {}

pub struct Transform {
    pub position: Vector3<f32>,
    pub quaternion: Quaternion<f32>,
    matrix: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            quaternion: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            matrix: Matrix4::identity(),
        }
    }
}

impl Updatable for Transform {
    fn update(&mut self) {
        self.matrix = Matrix4::from(self.quaternion) * Matrix4::from_translation(self.position);
    }
}

pub struct Mesh<'a> {
    pub transform: Transform,
    pub material: Material<'a>,
    pub geometry: Geometry,
}

impl<'a> Mesh<'a> {
    pub fn new(material: Material<'a>, geometry: Geometry) -> Mesh {
        Mesh {
            transform: Transform::new(),
            material: material,
            geometry: geometry,
        }
    }
}

impl GraphNode for Mesh<'_> {}

impl Updatable for Mesh<'_> {
    fn update(&mut self) {
        self.transform.update();
    }
}

impl Renderable for Mesh<'_> {
    fn render(&mut self, rendering_context: &RenderingContext) {
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
        let model_view_matrix: Matrix4<f32> = camera_matrix_invert * self.transform.matrix;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

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
            self.transform.matrix.as_ref() as &[f32; 16],
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
