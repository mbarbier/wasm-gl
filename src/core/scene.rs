use std::{cell::RefCell, rc::Rc};

use super::graph::Node;
use super::object3d::Object3d;

pub struct Scene {
    pub root: Rc<Node<Object3d>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            root: Rc::new(Node::new(Object3d::new())),
        }
    }

    pub fn add_child(&mut self, object: &Rc<Node<Object3d>>) {
        Node::add_child(&self.root, object);
    }
}
