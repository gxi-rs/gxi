mod comp {
    use std::cell::RefCell;
    use std::rc::Rc;

    use gxi::{GxiComponent, Node};

    /// App is a component which is cloneable
    #[derive(Clone, Default, GxiComponent)]
    pub struct Comp {
        node: Rc<RefCell<Node>>,
    }
}

mod app {
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use std::rc::Rc;

    use gxi::{init_member, GxiComponent, InitType, Node, VNode};

    use crate::comp::Comp;

    pub enum Msg {
        FOO,
    }

    /// App is a component which is cloneable
    #[derive(Clone, Default, GxiComponent)]
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
            let _node = init_member(node_mut_borrow.deref_mut(), InitType::Child, || Comp::new());
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
}

mod text {
    use gxi::{GxiWidget, Node};

    /// App is a component which is cloneable
    #[derive(Default, GxiWidget)]
    pub struct Text {
        node: Node,
    }
}

#[cfg(test)]
mod tests {
    use gxi::VNode;

    use crate::app::App;

    #[test]
    fn main() {
        let mut app = App::new();
        app.render();
    }
}
