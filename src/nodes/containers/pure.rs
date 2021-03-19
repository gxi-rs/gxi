use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::ContainerExt;

use crate::nodes::node::{AsyncNode, Node};

pub struct Pure {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for Pure {
    impl_node_trait!();
    impl_node_trait_get_child!();
    impl_node_trait_init_sibling!();

    fn init_child(&mut self, f: Box<dyn Fn() -> AsyncNode>, add_widget: bool) -> (AsyncNode, bool) {
        match self.child {
            None => {
                let child = self.child.get_or_insert_with(|| f());
                if add_widget {
                    let child_borrow = child.as_ref().borrow();
                    let parent_borrow = self.parent.as_ref().borrow();
                    parent_borrow
                        .get_widget_as_container()
                        .add(&child_borrow.get_widget());
                }
                (child.clone(), true)
            }
            _ => (self.child.as_ref().unwrap().clone(), false),
        }
    }

    fn get_widget(&self) -> gtk::Widget {
        let parent_borrow = self.parent.as_ref().borrow();
        parent_borrow.get_widget()
    }

    fn get_widget_as_container(&self) -> gtk::Container {
        let parent_borrow = self.parent.as_ref().borrow();
        parent_borrow.get_widget_as_container()
    }
}

impl Pure {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Pure {
            child: None,
            sibling: None,
            parent,
        })))
    }
}
