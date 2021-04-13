use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

pub type NodeRc<T> where T: Sized = Rc<RefCell<Box<T>>>;
pub type WeakNodeRc<T> where T: Sized = Weak<RefCell<Box<T>>>;

#[allow(drop_bounds)]
pub trait Node: Drop {
    type NativeWidget;
    type NativeWidgetContainer;

    fn get_child(&self) -> &Option<NodeRc<Self>> where Self: Sized {
        unimplemented!()
    }
    fn get_child_mut(&mut self) -> &mut Option<NodeRc<Self>> where Self: Sized {
        unimplemented!()
    }
    fn get_sibling(&self) -> &Option<NodeRc<Self>> where Self: Sized {
        unimplemented!()
    }
    fn get_sibling_mut(&mut self) -> &mut Option<NodeRc<Self>> where Self: Sized {
        unimplemented!()
    }
    fn init_child(&mut self, _f: Box<dyn FnOnce() -> NodeRc<Self>>) -> (NodeRc<Self>, bool) where Self: Sized ;
    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc<Self>>) -> (NodeRc<Self>, bool) where Self: Sized ;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> Self::NativeWidget;
    fn get_widget_as_container(&self) -> Self::NativeWidgetContainer;
    fn get_type(&self) -> NodeType { NodeType::Widget }
    fn new(parent: WeakNodeRc<Self>) -> NodeRc<Self> where Self: Sized;
    fn render(_this: NodeRc<Self>) where Self: Sized, {}
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&mut self);
    fn mark_clean(&mut self);
    // parent substitute is the the parent in which outer children are added
    fn get_self_substitute(&self) -> NodeRc<Self> where Self: Sized;
    fn set_self_substitute(&mut self, self_substitute: NodeRc<Self>) where Self: Sized ;
    //adds the widget of child to self widget
    //this method allow to draw clear lines between
    //OS specific and component system specific code
    fn add(&mut self, child: NodeRc<Self>) where Self: Sized ;
}
