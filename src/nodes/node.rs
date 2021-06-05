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

pub type NodeRc = Rc<RefCell<Box<dyn Node>>>;
pub type WeakNodeRc = Weak<RefCell<Box<dyn Node>>>;

#[cfg(feature = "gxi-desktop")]
pub type NativeWidget = gtk::Widget;

#[cfg(feature = "gxi-web")]
pub type NativeWidget = web_sys::Element;

#[allow(drop_bounds)]
pub trait Node: Drop {
    fn get_child(&self) -> &Option<NodeRc> { unreachable!() }
    fn get_child_mut(&mut self) -> &mut Option<NodeRc> { unreachable!() }
    fn get_sibling(&self) -> &Option<NodeRc> {
        unreachable!()
    }
    fn get_sibling_mut(&mut self) -> &mut Option<NodeRc> {
        unreachable!()
    }
    fn get_parent(&self) -> NodeRc { unreachable!() }
    fn init_child(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        match self.get_child() {
            None => {
                let child = self.get_child_mut().get_or_insert(f()).clone();
                if let NodeType::Widget = child.as_ref().borrow().get_type() {
                    self.add(child.clone());
                }
                (child, true)
            }
            Some(child) => (child.clone(), false),
        }
    }
    fn init_sibling(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        match self.get_sibling() {
            None => {
                {
                    self.get_sibling_mut().get_or_insert(f());
                }
                let sibling = self.get_sibling().as_ref().unwrap();
                if let NodeType::Widget = sibling.as_ref().borrow().get_type() {
                    self.get_parent().borrow_mut().add(sibling.clone());
                }
                (sibling.clone(), true)
            }
            Some(sibling) => (sibling.clone(), false),
        }
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> NativeWidget;
    fn get_type(&self) -> NodeType {
        NodeType::Widget
    }
    fn new(parent: WeakNodeRc) -> NodeRc
        where
            Self: Sized;
    fn render(_this: NodeRc)
        where
            Self: Sized,
    {}
    fn is_dirty(&self) -> bool {
        false
    }
    fn mark_dirty(&mut self) {}
    fn mark_clean(&mut self) {}
    // parent substitute is the the parent in which outer children are added
    fn get_self_substitute(&self) -> NodeRc;
    fn set_self_substitute(&mut self, self_substitute: NodeRc);
    //adds the widget of child to self widget
    //this method allow to draw clear lines between
    //OS specific and component system specific code
    fn add(&mut self, child: NodeRc);
}
