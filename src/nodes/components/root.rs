use std::any::Any;

use crate::{*};
use std::cell::RefCell;
use std::rc::Rc;

const PANIC_MSG: &str = "You can't call any function on GxiTree. GxiTree can only used as root";

pub struct Root {
    pub child: Option<NodeRc>,
}

impl Node for Root {
    impl_node_trait_get_child!();

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        panic!("{}", PANIC_MSG);
    }

    fn as_any(&self) -> &dyn Any {
        panic!("{}", PANIC_MSG);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        panic!("{}", PANIC_MSG);
    }

    fn get_widget(&self) -> NativeWidget {
        panic!("{}", PANIC_MSG);
    }

    fn new(_parent: WeakNodeRc) -> NodeRc
        where
            Self: Sized,
    {
        panic!("{}", PANIC_MSG);
    }

    fn is_dirty(&self) -> bool {
        panic!("{}", PANIC_MSG);
    }

    fn mark_dirty(&mut self) {
        panic!("{}", PANIC_MSG);
    }

    fn mark_clean(&mut self) {
        panic!("{}", PANIC_MSG);
    }

    fn get_self_substitute(&self) -> NodeRc {
        panic!("{}", PANIC_MSG);
    }

    fn set_self_substitute(&mut self, _self_substitute: NodeRc) {
        panic!("{}", PANIC_MSG);
    }

    fn add(&mut self, _child: NodeRc) { panic!("{}", PANIC_MSG); }
}

impl Root {
    pub fn new_root() -> NodeRc {
        Rc::new(RefCell::new(Box::new(Self {
            child: None
        })))
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        // Components need to not drop anything
    }
}
