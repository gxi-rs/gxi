use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*};

pub struct MyApp {
    count: i32,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for MyApp {
    impl_node_trait!();
    impl_node_trait_get_child!();
    impl_node_trait_init_sibling!();

    fn init_child(&mut self, f: Box<dyn Fn() -> AsyncNode>, add_widget: bool) -> (AsyncNode, bool) {
        match self.child {
            None => {
                let child = self.child.get_or_insert_with(|| f());
                if add_widget {
                    let child_borrow = child.as_ref().borrow();
                    let parent_borrow = self.parent.as_ref().borrow();
                    let container = parent_borrow.get_widget_as_container();
                    container.add(&child_borrow.get_widget());
                    container.show_all();
                }
                (child.clone(), true)
            }
            _ => (self.child.as_ref().unwrap().clone(), false),
        }
    }

    fn get_widget(&self) -> gtk::Widget {
        let parent_borrow = self.parent.as_ref().borrow();
        parent_borrow.get_widget()
    }

    fn get_widget_as_container(&self) -> gtk::Container {
        let parent_borrow = self.parent.as_ref().borrow();
        parent_borrow.get_widget_as_container()
    }
    fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            count: 0,
            child: None,
            sibling: None,
            parent,
        })))
    }

    fn render(top_state: AsyncNode) {
        let container = top_state.as_ref().borrow().as_any().downcast_ref::<Self>().unwrap().parent.clone();
        let cont = Rc::clone(&container);
        let node = cont.clone();
        c!(
            View [
                View [
                    {
                        if state.count % 2 == 0 {
                            c!(1 Button { set_label = "even"; connect_clicked = || state.count += 1; });
                        } else {
                            c!(2 View);
                        }
                    }
                    Button { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; },
                ]
            ]
        );
    }
}