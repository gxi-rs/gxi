use std::rc::{Rc, Weak};

use std::cell::RefCell;
use crate::{*};

pub type StrongNodeType = Rc<RefCell<GxiNodeType>>;
pub type WeakNodeType = Weak<RefCell<GxiNodeType>>;

pub enum GxiNodeType {
    Widget(Box<dyn WidgetNode>),
    Component(Box<dyn ComponentNode>),
    Container(Box<dyn ContainerNode>),
}
