#[macro_export]
macro_rules! impl_node_trait {
    () => {
        fn get_child(&self) -> &Option<Arc<Mutex<Node>>> {
            &self.child
        }

        fn get_sibling(&self) -> &Option<Arc<Mutex<Node>>> {
            &self.sibling
        }

        fn get_child_mut(&mut self) -> &mut Option<Arc<Mutex<Node>>> {
            &mut self.child
        }

        fn get_sibling_mut(&mut self) -> &mut Option<Arc<Mutex<Node>>> {
            &mut self.sibling
        }
    };
}
