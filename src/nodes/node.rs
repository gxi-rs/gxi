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
