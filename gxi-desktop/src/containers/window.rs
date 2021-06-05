use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::WindowType;

use crate::*;

pub struct Window {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_for_widget_component!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: gtk::Window::new(WindowType::Toplevel),
        })));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }
        this
    }

    fn render(state: NodeRc) {
        let mut state = state.as_ref().borrow_mut();
        let state = state.as_any_mut().downcast_mut::<Self>().unwrap();
        if state.dirty {
            state.widget.show_all();
        }
        state.mark_clean();
    }

    impl_node_trait_dirty!();
    impl_add_for_desktop_node!();
}

impl Window {
    pub fn on_destroy<F: Fn() + 'static>(&self, f: F) {
        self.widget.connect_destroy(move |_| f());
    }
}

impl_drop_for_node!(Window);
