use std::rc::{Rc, Weak};

use crate::*;
use std::cell::RefCell;

pub type StrongNodeType = Rc<RefCell<GxiNodeType>>;
pub type WeakNodeType = Weak<RefCell<GxiNodeType>>;

pub enum GxiNodeType {
    /// node which can hold a reference to native widget
    Widget(Box<dyn WidgetNode>),
    /// node which itself is a widget and can hold other widgets
    ContainerWidget(Box<dyn ContainerWidgetNode>),
    /// node which itself is a widget and can hold other widgets but can't be added to other widgets
    /// eg. Window. Window is a top level widget which can't be added to any other widgets like button
    TopLevelWidget(Box<dyn ContainerWidgetNode>),
    /// container without any native widget
    Component(Box<dyn ComponentNode>),
}

impl GxiNodeType {
    pub fn as_node(&self) -> &dyn Node {
        match self {
            GxiNodeType::ContainerWidget(this) => this.as_node(),
            GxiNodeType::Widget(this) => this.as_node(),
            GxiNodeType::Component(this) => this.as_node(),
            GxiNodeType::TopLevelWidget(this) => this.as_node()
        }
    }

    pub fn as_node_mut(&mut self) -> &mut dyn Node {
        match self {
            GxiNodeType::ContainerWidget(this) => this.as_node_mut(),
            GxiNodeType::Widget(this) => this.as_node_mut(),
            GxiNodeType::Component(this) => this.as_node_mut(),
            GxiNodeType::TopLevelWidget(this) => this.as_node_mut(),
        }
    }

    pub fn as_widget_node(&self) -> Result<&dyn WidgetNode, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(this) => Ok(this.as_widget_node()),
            GxiNodeType::Widget(this) => Ok(this.as_widget_node()),
            GxiNodeType::Component(_) => Err("can't convert ComponentNode to WidgetNode"),
            GxiNodeType::TopLevelWidget(this) => Ok(this.as_widget_node()),
        }
    }

    pub fn as_widget_node_mut(&mut self) -> Result<&mut dyn WidgetNode, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(this) => Ok(this.as_widget_node_mut()),
            GxiNodeType::TopLevelWidget(this) => Ok(this.as_widget_node_mut()),
            GxiNodeType::Widget(this) => Ok(this.as_widget_node_mut()),
            GxiNodeType::Component(_) => Err("can't convert ComponentNode to WidgetNode"),
        }
    }

    pub fn as_container(&self) -> Result<&dyn Container, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(this) => Ok(this.as_container()),
            GxiNodeType::TopLevelWidget(this) => Ok(this.as_container()),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode to Container"),
            GxiNodeType::Component(this) => Ok(this.as_container()),
        }
    }

    pub fn as_container_mut(&mut self) -> Result<&mut dyn Container, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(this) => Ok(this.as_container_mut()),
            GxiNodeType::TopLevelWidget(this) => Ok(this.as_container_mut()),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode to Container"),
            GxiNodeType::Component(this) => Ok(this.as_container_mut()),
        }
    }

    pub fn as_component_node(&self) -> Result<&dyn ComponentNode, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(_) => Err("can't ContainerWidgetNode widget to ComponentNode"),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode to ComponentNode"),
            GxiNodeType::Component(this) => Ok(this.as_ref()),
            GxiNodeType::TopLevelWidget(_) => Err("can't convert TopLevelWidgetNode to ComponentNode"),
        }
    }

    pub fn as_component_node_mut(&mut self) -> Result<&mut dyn ComponentNode, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(_) => Err("can't ContainerWidgetNode widget to ComponentNode"),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode to ComponentNode"),
            GxiNodeType::Component(this) => Ok(this.as_mut()),
            GxiNodeType::TopLevelWidget(_) => Err("can't convert TopLevelWidgetNode to ComponentNode"),
        }
    }

    pub fn as_container_widget_node(&self) -> Result<&dyn ContainerWidgetNode, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(this) => Ok(this.as_ref()),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode to ContainerWidgetNode"),
            GxiNodeType::Component(_) => Err("can't convert ComponentNode to ContainerWidgetNode"),
            GxiNodeType::TopLevelWidget(this) => Ok(this.as_ref()),
        }
    }

    pub fn as_container_widget_node_mut(&mut self) -> Result<&mut dyn ContainerWidgetNode, &'static str> {
        match self {
            GxiNodeType::ContainerWidget(this) => Ok(this.as_mut()),
            GxiNodeType::Widget(_) => Err("can't convert WidgetNode to ContainerWidgetNode"),
            GxiNodeType::Component(_) => Err("can't convert ComponentNode to ContainerWidgetNode"),
            GxiNodeType::TopLevelWidget(this) => Ok(this.as_mut()),
        }
    }
}
