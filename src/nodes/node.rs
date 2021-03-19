use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub type AsyncNode = Rc<RefCell<Box<dyn Node>>>;

pub trait Node {
    fn get_child(&self) -> &Option<AsyncNode> {
        unimplemented!()
    }
    fn get_child_mut(&mut self) -> &mut Option<AsyncNode> {
        unimplemented!()
    }
    fn get_sibling(&self) -> &Option<AsyncNode> {
        unimplemented!()
    }
    fn get_sibling_mut(&mut self) -> &mut Option<AsyncNode> {
        unimplemented!()
    }
    fn init_child(&mut self, _f: Box<dyn Fn() -> AsyncNode>) -> (AsyncNode, bool) { unimplemented!() }
    fn init_sibling(&mut self, _f: Box<dyn Fn() -> AsyncNode>) -> (AsyncNode, bool) { unimplemented!() }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> gtk::Widget;
    fn get_widget_as_container(&self) -> gtk::Container;
}
