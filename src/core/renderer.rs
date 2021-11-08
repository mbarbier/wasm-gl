use std::{cell::RefCell, rc::Rc};

use cgmath::{perspective, Deg, Matrix4};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGl2RenderingContext;
use weblog::console_log;

use super::{object3d::Node, scene::Scene};

pub struct Renderer {
    pub canvas: web_sys::HtmlCanvasElement,

    context: WebGl2RenderingContext,
    width: f64,
    height: f64,
}

impl Renderer {
    pub fn new(canvas_id: &str) -> Renderer {
        let window = web_sys::window().expect("No global window object");
        let document = window.document().expect("Should have a document on window");

        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("No canvas found")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        Renderer {
            canvas,
            context,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.width = width;
        self.height = height;

        self.context.viewport(0, 0, width as i32, height as i32);
    }

    pub fn create(&mut self) -> Result<(), JsValue> {
        self.context.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.context.enable(WebGl2RenderingContext::CULL_FACE);
        self.context.depth_func(WebGl2RenderingContext::LEQUAL);

        console_log!("Context initialized");

        Ok(())
    }

    pub fn draw(&self, scene: &mut Scene, camera: &Matrix4<f32>, _dt: f32) -> Result<(), JsValue> {
        update_rec(&scene.root);

        let rendering_context = RenderingContext {
            gl: &self.context,
            projection_matrix: &perspective(Deg(45.0), (self.width / self.height) as f32, 0.1, 100.0),
            camera_matrix: camera,
        };

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear_depth(1.);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        render_rec(&scene.root, &rendering_context);

        Ok(())
    }
}

fn update_rec(node: &Rc<RefCell<Node>>) {
    node.borrow_mut().update();

    // console_log!(format!("update {:?} {:?}", n.name.as_ref(), n.transform.matrix_world));
    node.as_ref().borrow().children.iter().for_each(|f| {
        update_rec(f);
    });
}

fn render_rec(node: &Rc<RefCell<Node>>, rendering_context: &RenderingContext) {
    let n = node.borrow_mut();
    if n.renderer.is_some() {
        // console_log!("render ", n.name.as_ref());
        n.renderer
            .as_ref()
            .unwrap()
            .borrow_mut()
            .render(&n.transform, rendering_context);
    }
    n.children.iter().for_each(|f| {
        render_rec(f, rendering_context);
    });
}

pub struct RenderingContext<'a, 'b> {
    pub gl: &'a WebGl2RenderingContext,
    pub projection_matrix: &'b Matrix4<f32>,
    pub camera_matrix: &'b Matrix4<f32>,
}
