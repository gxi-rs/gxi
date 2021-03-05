#[macro_export]
macro_rules! impl_node_trait {
    () => {
        fn get_child(&self) -> &Option<Rc<RefCell<Node>>> {
        &self.child
    }

    fn get_sibling(&self) -> &Option<Rc<RefCell<Node>>> {
        &self.sibling
    }

    fn get_child_mut(&mut self) -> &mut Option<Rc<RefCell<Node>>> {
        &mut self.child
    }

    fn get_sibling_mut(&mut self) -> &mut Option<Rc<RefCell<Node>>> {
        &mut self.sibling
    }
    };
}