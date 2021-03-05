use crate::nodes::{component, container, widget};

pub enum Node {
    Widget(Box<dyn widget::Widget>),
    Container(Box<dyn container::Container>),
    Component(Box<dyn component::Component>),
}

pub trait NodeTrait {
    fn get_child(&self) -> &Option<Node>;
    fn get_sibling(&self) -> &Option<Node>;
    fn get_child_mut(&mut self) -> &mut Option<Node>;
    fn get_sibling_mut(&mut self) -> &mut Option<Node>;
}

#[macro_export]
macro_rules! impl_node_trait {
    () => {
        fn get_child(&self) -> &Option<Node> {
            &self.child
        }

        fn get_sibling(&self) -> &Option<Node> {
            &self.sibling
        }

        fn get_child_mut(&mut self) -> &mut Option<Node> {
            &mut self.child
        }

        fn get_sibling_mut(&mut self) -> &mut Option<Node> {
            &mut self.sibling
        }
    };
}
