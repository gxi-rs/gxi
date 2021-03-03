mod node {
    use std::cell::RefCell;

    use crate::comp::{component, container, widget};

    pub enum Node {
        Widget(Box<dyn widget::Widget>),
        Container(Box<dyn container::Container>),
        Component(Box<dyn component::Component>),
    }

    pub trait NodeTrait {
        fn get_child(&self) -> &Option<Node>;
        fn get_sibling(&self) -> &Option<Node>;
    }
}

mod container {
    use crate::comp::node::NodeTrait;

    pub trait Container: NodeTrait {}
}

mod widget {
    use crate::comp::node::NodeTrait;

    pub trait Widget: NodeTrait {}
}

mod component {
    use crate::comp::node::NodeTrait;

    pub trait Component: NodeTrait {}
}

mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::comp::node::Node;
    use crate::comp::container::Container;

    mod views {
        use crate::comp::container::Container;
        use crate::comp::node::{Node, NodeTrait};

        #[derive(Default)]
        pub struct Grid {
            pub child: Option<Node>
        }

        impl Container for Grid {}

        impl NodeTrait for Grid {
            fn get_child(&self) -> &Option<Node> {
                &self.child
            }

            fn get_sibling(&self) -> &Option<Node> {
                unimplemented!()
            }
        }
    }

    mod widgets {
        use std::cell::RefCell;
        use std::rc::Rc;

        use crate::comp::container::Container;
        use crate::comp::node::{Node, NodeTrait};
        use crate::comp::widget::Widget;

        pub struct Button {
            pub parent: Rc<RefCell<Box<dyn Container>>>
        }

        impl NodeTrait for Button {
            fn get_child(&self) -> &Option<Node> {
                unimplemented!()
            }

            fn get_sibling(&self) -> &Option<Node> {
                unimplemented!()
            }
        }

        impl Widget for Button {}
    }

    #[test]
    fn test() {
        let container: Rc<RefCell<Box<dyn Container>>> = Rc::new(RefCell::new(Box::new(views::Grid::default())));
        let container_clone = container.clone().get_mut();
        container_clone.get_sibling().get_or_insert_with(|| Node::Widget(Box::new(widgets::Button { parent: container.clone() })));
    }
}
