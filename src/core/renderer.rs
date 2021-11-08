use cgmath::{perspective, Deg, Matrix4};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGl2RenderingContext;
use weblog::console_log;

use super::scene::Scene;

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
        scene.objects.iter().for_each(|node| {
            node.borrow_mut().update();
        });

        let rendering_context = RenderingContext {
            gl: &self.context,
            projection_matrix: &perspective(Deg(45.0), (self.width / self.height) as f32, 0.1, 100.0),
            camera_matrix: camera,
        };

        scene.objects.iter().for_each(|node| {
            node.borrow_mut().render(&rendering_context);
        });

        Ok(())
    }
}

pub struct RenderingContext<'a, 'b> {
    pub gl: &'a WebGl2RenderingContext,
    pub projection_matrix: &'b Matrix4<f32>,
    pub camera_matrix: &'b Matrix4<f32>,
}
