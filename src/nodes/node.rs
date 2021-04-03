use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeRc = Rc<RefCell<Box<dyn Node>>>;
pub type WeakNodeRc = Weak<RefCell<Box<dyn Node>>>;

pub enum NodeType {
    Widget,
    Component,
}

impl NodeType {
    pub fn should_add_widget(&self) -> bool {
        match self {
            NodeType::Widget => true,
            NodeType::Component => false,
        }
    }
}

pub trait Node: Drop {
    fn get_child(&self) -> &Option<NodeRc> {
        unimplemented!()
    }
    fn get_child_mut(&mut self) -> &mut Option<NodeRc> {
        unimplemented!()
    }
    fn get_sibling(&self) -> &Option<NodeRc> {
        unimplemented!()
    }
    fn get_sibling_mut(&mut self) -> &mut Option<NodeRc> {
        unimplemented!()
    }
    fn init_child(
        &mut self, _f: Box<dyn FnOnce() -> NodeRc>, parent_container: gtk::Container,
    ) -> (NodeRc, bool);
    fn init_sibling(
        &mut self, _f: Box<dyn FnOnce() -> NodeRc>, _parent_container: gtk::Container,
    ) -> (NodeRc, bool);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> gtk::Widget;
    fn get_widget_as_container(&self) -> gtk::Container;
    fn get_type(&self) -> NodeType {
        NodeType::Widget
    }
    fn new(parent_widget: Option<gtk::Container>) -> NodeRc where Self: Sized;
    fn render(_top_state: NodeRc) where Self: Sized {}
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&mut self);
    fn mark_clean(&mut self);
}
