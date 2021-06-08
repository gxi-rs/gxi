use std::rc::{Rc, Weak};

use crate::*;
use std::cell::RefCell;

pub type StrongNodeType = Rc<RefCell<GxiNodeType>>;
pub type WeakNodeType = Weak<RefCell<GxiNodeType>>;

pub enum GxiNodeType {
    Widget(Box<dyn WidgetNode>),
    Component(Box<dyn ComponentNode>),
    Container(Box<dyn ContainerNode>),
}

impl GxiNodeType {
    pub fn as_node(&self) -> &dyn Node {
        match self {
            GxiNodeType::Container(this) => this.as_node(),
            GxiNodeType::Widget(this) => this.as_node(),
            GxiNodeType::Component(this) => this.as_node(),
        }
    }

    pub fn as_node_mut(&mut self) -> &mut dyn Node {
        match self {
            GxiNodeType::Container(this) => this.as_node_mut(),
            GxiNodeType::Widget(this) => this.as_node_mut(),
            GxiNodeType::Component(this) => this.as_node_mut(),
        }
    }

    pub fn as_widget_node(&self) -> Result<&dyn WidgetNode, &'static str> {
        match self {
            GxiNodeType::Container(this) => Ok(this.as_widget_node()),
            GxiNodeType::Widget(this) => Ok(this.as_widget_node()),
            GxiNodeType::Component(_) => Err("can't convert ComponentNode to WidgetNode"),
        }
    }

    pub fn as_widget_node_mut(&mut self) -> Result<&mut dyn WidgetNode, &'static str> {
        match self {
            GxiNodeType::Container(this) => Ok(this.as_widget_node_mut()),
            GxiNodeType::Widget(this) => Ok(this.as_widget_node_mut()),
            GxiNodeType::Component(_) => Err("can't convert ComponentNode to WidgetNode"),
        }
    }
    
    pub fn as_container(&self) -> Result<&dyn Container, &'static str> {
        match self {
            GxiNodeType::Container(this) => Ok(this.as_container()),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode Container"),
            GxiNodeType::Component(this) => Ok(this.as_container()),
        }
    }

    pub fn as_container_mut(&mut self) -> Result<&mut dyn Container, &'static str> {
        match self {
            GxiNodeType::Container(this) => Ok(this.as_container_mut()),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode Container"),
            GxiNodeType::Component(this) => Ok(this.as_container_mut()),
        }                                                               
    }
}
