use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T> {
    pub value: RefCell<T>,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            value: RefCell::new(data),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub fn new_rc(data: T) -> Rc<Node<T>> {
        let node = Node::new(data);
        Rc::new(node)
    }

    pub fn add_child(parent: &Rc<Node<T>>, child: &Rc<Node<T>>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child.clone());
    }

    pub fn traverse(node: &Rc<Node<T>>, action: &TraverseCb<T>) {
        action(node);
        node.children.borrow().iter().for_each(|n| Node::traverse(n, action));
    }
}

type TraverseCb<'a, T> = dyn Fn(&Rc<Node<T>>) -> () + 'a;

impl<T> Node<T>
where
    T: Debug,
{
    pub fn print(node: &Rc<Node<T>>) {
        Node::traverse(node, &|n: &Rc<Node<T>>| {
            println!("{:?}", n.value.borrow());
        });
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_0() {
        let root = Node::new_rc("the_root".to_string());
        let child_0 = Node::new_rc("child_0".to_string());
        let child_0_0 = Node::new_rc("child_0_0".to_string());
        let child_0_1 = Node::new_rc("child_0_1".to_string());
        let child_1 = Node::new_rc("child_1".to_string());

        Node::add_child(&root, &child_0);
        Node::add_child(&root, &child_1);

        Node::add_child(&child_0, &child_0_0);
        Node::add_child(&child_0, &child_0_1);

        Node::traverse(&root, &|node: &Rc<Node<String>>| {
            println!(
                "{}, strong = {}, weak = {}",
                node.value.borrow(),
                Rc::strong_count(node),
                Rc::weak_count(node),
            );
        });

        Node::traverse(&child_0, &|node: &Rc<Node<String>>| {
            let new_value = format!("{}_new_value", node.value.borrow());
            node.value.replace(new_value);
        });

        Node::print(&root);
    }
}
