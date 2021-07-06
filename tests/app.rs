mod comp {
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;

    use gxi::{Node, VComponent, VNode, VNodeType};

    /// App is a component which is cloneable
    #[derive(Clone, Default)]
    pub struct Comp {
        state: Rc<RefCell<AppState>>,
        node: Rc<RefCell<Node>>,
    }

    #[derive(Default)]
    struct AppState {}

    impl VNode for Comp {
        fn new() -> Self { Self::default() }

        fn into_vnode_type(self) -> VNodeType where Self: Sized {
            VNodeType::Component(Box::from(self))
        }
    }

    impl AsRef<dyn VNode> for Comp {
        fn as_ref(&self) -> &dyn VNode {
            self
        }
    }

    impl AsMut<dyn VNode> for Comp {
        fn as_mut(&mut self) -> &mut dyn VNode {
            self
        }
    }

    impl AsRef<dyn Any> for Comp {
        fn as_ref(&self) -> &dyn Any {
            self
        }
    }

    impl AsMut<dyn Any> for Comp {
        fn as_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl VComponent for Comp {
        fn get_node_ref(&self) -> &RefCell<Node> {
            self.node.as_ref()
        }
    }
}


mod app {
    use std::any::Any;
    use std::cell::RefCell;
    use std::ops::{Deref, DerefMut};
    use std::rc::Rc;

    use gxi::{init_member, InitType, Node, VComponent, VNode, VNodeType};

    use crate::comp::Comp;

    enum Msg {
        FOO
    }

    /// App is a component which is cloneable
    #[derive(Clone, Default)]
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

    // impl deref for app state
    impl Deref for App {
        type Target = RefCell<AppState>;

        fn deref(&self) -> &Self::Target {
            self.state.as_ref()
        }
    }

    impl VNode for App {
        fn new() -> Self { Self::default() }

        fn into_vnode_type(self) -> VNodeType where Self: Sized {
            VNodeType::Component(Box::from(self))
        }
    }

    impl AsRef<dyn VNode> for App {
        fn as_ref(&self) -> &dyn VNode {
            self
        }
    }

    impl AsMut<dyn VNode> for App {
        fn as_mut(&mut self) -> &mut dyn VNode {
            self
        }
    }

    impl AsRef<dyn Any> for App {
        fn as_ref(&self) -> &dyn Any {
            self
        }
    }

    impl AsMut<dyn Any> for App {
        fn as_mut(&mut self) -> &mut dyn Any {
            self
        }
    }


    impl VComponent for App {
        fn get_node_ref(&self) -> &RefCell<Node> {
            self.node.as_ref()
        }
    }
}

mod text {
    use std::any::Any;

    use gxi::{Node, VNode, VNodeType, VWidget};

    /// App is a component which is cloneable
    #[derive(Default)]
    pub struct Text {
        node: Node,
    }

    impl VNode for Text {
        fn new() -> Self { Self::default() }

        fn into_vnode_type(self) -> VNodeType where Self: Sized {
            VNodeType::Widget(Box::from(self))
        }
    }

    impl AsRef<dyn VNode> for Text {
        fn as_ref(&self) -> &dyn VNode {
            self
        }
    }

    impl AsMut<dyn VNode> for Text {
        fn as_mut(&mut self) -> &mut dyn VNode {
            self
        }
    }

    impl AsRef<dyn Any> for Text {
        fn as_ref(&self) -> &dyn Any {
            self
        }
    }

    impl AsMut<dyn Any> for Text {
        fn as_mut(&mut self) -> &mut dyn Any {
            self
        }
    }


    impl VWidget for Text {
        fn get_node(&self) -> &Node {
            &self.node
        }

        fn get_node_mut(&mut self) -> &mut Node {
            &mut self.node
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;
    use gxi::VNode;

    #[test]
    fn main() {
        let mut app = App::new();
        app.render();
    }
}
