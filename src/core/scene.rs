use std::{cell::RefCell, rc::Rc};

use super::object3d::Node;

pub struct Scene {
    pub root: Rc<RefCell<Node>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            root: Rc::new(RefCell::new(Node::new())),
        }
    }

    pub fn add_child(&mut self, object: Rc<RefCell<Node>>) {
        self.root.borrow_mut().add_child(self.root.clone(), object);
    }
}
