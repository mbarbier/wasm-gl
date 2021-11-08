use std::{cell::RefCell, rc::Rc};

use super::object3d::GraphNode;

pub struct Scene {
    // pub objects: Vec<Box<dyn GraphNode>>,
    pub objects: Vec<Rc<RefCell<dyn GraphNode>>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new() }
    }

    // pub fn add(&mut self, object: Box<dyn GraphNode>) {
    //     self.objects.push(object);
    // }

    pub fn add(&mut self, object: Rc<RefCell<dyn GraphNode>>) {
        self.objects.push(object);
    }
}
