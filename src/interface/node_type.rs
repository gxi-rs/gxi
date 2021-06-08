use std::rc::{Rc, Weak};

use std::cell::RefCell;
use crate::{*};
use std::ops::Deref;

pub type StrongNodeType = Rc<RefCell<GxiNodeType>>;
pub type WeakNodeType = Weak<RefCell<GxiNodeType>>;

pub enum GxiNodeType {
    Widget(Box<dyn WidgetNode>),
    Component(Box<dyn ComponentNode>),
    Container(Box<dyn ContainerNode>),
}

impl GxiNodeType {
    pub fn to_node(&self) -> &dyn Node {
        match self {
            GxiNodeType::Container(this) => this.to_node(),
            GxiNodeType::Widget(this) => this.to_node(),
            GxiNodeType::Component(this) => this.to_node(),
        }
    }
}
