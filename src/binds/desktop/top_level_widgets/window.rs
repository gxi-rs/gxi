use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

use gtk::{WindowType, ContainerExt, WidgetExt};

use crate::*;

pub struct Window {
    parent: WeakNodeType,
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    widget: gtk::Window,
    self_substitute: Option<WeakNodeType>,
}

impl Node for Window {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::TopLevelWidget(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: {
                let window = gtk::Window::new(WindowType::Toplevel);
                window.show_all();
                window
            },
            self_substitute: None,
        }))))
    }

    fn render(this: StrongNodeType) {
        let this = this.as_ref().borrow();
        let this = this.as_container_widget_node().unwrap();
        this.get_native_container().show_all();
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl_container_node!(Window);
impl_component_node!(Window);
impl_container!(Window);
impl_widget_node!(Window);
impl_drop!(Window);
impl_widget_node_deref!(Window Window);

