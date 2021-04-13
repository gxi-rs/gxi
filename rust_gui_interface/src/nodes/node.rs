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

#[allow(drop_bounds)]
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
    fn init_child(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool);
    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> NativeWidget;
    fn get_widget_as_container(&self) -> NativeWidgetContainer;
    fn get_type(&self) -> NodeType {
        NodeType::Widget
    }
    fn new(parent: WeakNodeRc) -> NodeRc
    where
        Self: Sized;
    fn render(_this: NodeRc)
    where
        Self: Sized,
    {
    }
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&mut self);
    fn mark_clean(&mut self);
    // parent substitute is the the parent in which outer children are added
    fn get_self_substitute(&self) -> NodeRc;
    fn set_self_substitute(&mut self, self_substitute: NodeRc);
    //adds the widget of child to self widget
    //this method allow to draw clear lines between
    //OS specific and component system specific code
    fn add(&mut self, child: NodeRc);
}
