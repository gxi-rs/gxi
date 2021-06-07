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

/*impl Deref for GxiNodeType {
    type Target = dyn Node;

    fn deref(&self) -> &Self::Target {
        match self {
            GxiNodeType::Container(this) => this.deref(),
            GxiNodeType::Widget(this) => this.deref(),
            GxiNodeType::Component(this) => this.deref(),
        }
    }
}*/
/*
impl Deref for GxiNodeType {
    type Target = dyn Container;

    fn deref(&self) -> &Self::Target {
        match self {
            GxiNodeType::Widget(this) => panic!("Can't deref GxiNodeType into Container"),
            GxiNodeType::Component(this) => this.as_ref(),
            GxiNodeType::Container(this) => this.as_ref()
        }
    }
}
*/

impl GxiNodeType {
    pub fn into_node(self) -> Box<dyn Node> {
        match self {
            GxiNodeType::Widget(this) => this.into(),
            GxiNodeType::Component(this) => this.into(),
            GxiNodeType::Container(this) => this.into()
        }
    }
}

/*impl GxiNodeType {
    pub fn into_node(&self) -> &Box<dyn Node> {
        match self {
            GxiNodeType::Widget(this) => panic!("Can't deref GxiNodeType into Container"),
            GxiNodeType::Component(this) => this,
            GxiNodeType::Container(this) => this.to_node()
        }
    }
}*/