use crate::*;
use gtk::ContainerExt;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct View {
    parent: WeakNodeType,
    self_substitute: Option<WeakNodeType>,
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    widget: gtk::Box,
}

impl Node for View {
    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();

    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::ContainerWidget(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: gtk::Box::new(gtk::Orientation::Horizontal, 0),
        }))))
    }
}

impl_container_node!(View);
impl_container!(View);
impl_widget_node!(View);
impl_component_node!(View);
impl_drop!(View);
impl_widget_node_deref!(View gtk::Box);
