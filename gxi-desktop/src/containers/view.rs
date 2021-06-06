use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use gxi::{GxiNodeRc, WeakNodeType, NodeType, Node, impl_node_trait_as_any, impl_node_trait_dirty, WidgetNode};
use gtk::{WidgetExt, OrientableExt};
use crate::impl_widget::GtkContainer;
use crate::gxi::NativeWidget;


pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct View {
    pub parent: WeakNodeType,
    pub is_dirty: bool,
    pub self_substitute: Option<WeakNodeType>,
    pub child: Option<NodeType>,
    pub sibling: Option<NodeType>,
    pub widget: GtkContainer<gtk::Box>,
}

impl Node for View {
    impl_node_trait_as_any!();
    impl_node_trait_dirty!();
    //impl_node_trait_get_widget!();
    //impl_node_trait_get_sibling!();
    //impl_node_trait_get_child!();
    //impl_node_trait_get_parent!();
    //impl_add_for_desktop_node!();
    //impl_node_trait_substitute!();

    fn new(parent: WeakNodeType) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            is_dirty: true,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: GtkContainer(gtk::Box::new(gtk::Orientation::Horizontal, 0)),
        })));
        /*{
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }*/
        this
    }

    fn render(state: GxiNodeRc) {
        let mut state = state.as_ref().borrow_mut();
        let state = state.as_any_mut().downcast_mut::<Self>().unwrap();
        if state.dirty {
            state.widget.show_all();
        }
        state.mark_clean();
    }

    fn get_child(&self) -> &Option<NodeType> {
        todo!()
    }

    fn get_child_mut(&mut self) -> &mut Option<NodeType> {
        todo!()
    }

    fn get_parent(&self) -> WeakNodeType {
        todo!()
    }
}

impl WidgetNode for View {
    fn get_widget(&self) -> &dyn NativeWidget {
        &self.widget
    }

    fn get_widget_mut(&mut self) -> &mut dyn NativeWidget {
        &mut self.widget
    }
}

impl View {
    pub fn orientation(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Horizontal => {
                if let gtk::Orientation::Vertical = self.widget.get_orientation() {
                    self.widget.set_orientation(gtk::Orientation::Horizontal);
                }
            }
            Orientation::Vertical => {
                if let gtk::Orientation::Horizontal = self.widget.get_orientation() {
                    self.widget.set_orientation(gtk::Orientation::Vertical);
                }
            }
        }
    }

    pub fn h_expand(&mut self, h_expand: bool) {
        if h_expand != self.widget.get_hexpand() {
            self.widget.set_hexpand(true);
        }
    }

    pub fn v_expand(&mut self, v_expand: bool) {
        if v_expand != self.widget.get_vexpand() {
            self.widget.set_vexpand(true);
        }
    }
}

impl_drop_for_node!(View);
