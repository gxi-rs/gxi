use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use common::Comp;
use gxi::VNode;
use gxi::{init_member, InitType, Node};

mod common;

pub enum Msg {
    FOO,
}

/// App is a component which is cloneable
#[derive(Clone, Default, gxi::Component)]
pub struct App {
    state: Rc<RefCell<AppState>>,
    node: Rc<RefCell<Node>>,
}

#[derive(Default)]
pub struct AppState {
    name: String,
}

impl App {
    pub fn render(&mut self) {
        let mut node_mut_borrow = self.node.as_ref().borrow_mut();
        let mut _state = self.state.borrow_mut();
        let _node = init_member(node_mut_borrow.deref_mut(), InitType::Child, Comp::new);
    }
}

impl AppState {
    // update function of the component
    pub fn update(&self, msg: Msg) {
        match msg {
            Msg::FOO => {
                println!("my name is {}", &self.name)
            }
        }
    }

    pub fn name(&mut self, name: String) {
        self.name = name
    }
}

#[cfg(test)]
mod tests {
    use gxi::VNode;

    use crate::App;

    #[test]
    fn main() {
        let mut app = App::new();
        app.render();
    }
}
