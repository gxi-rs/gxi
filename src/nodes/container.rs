use crate::nodes::node::NodeTrait;

pub trait Container: NodeTrait {
    fn get_widget(&self) -> &gtk::Container;
}
