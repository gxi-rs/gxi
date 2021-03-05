#[macro_export]
macro_rules! impl_node_trait {
    () => {
        fn get_child(&self) -> &Option<AsyncNode> {
            &self.child
        }

        fn get_sibling(&self) -> &Option<AsyncNode> {
            &self.sibling
        }

        fn get_child_mut(&mut self) -> &mut Option<AsyncNode> {
            &mut self.child
        }

        fn get_sibling_mut(&mut self) -> &mut Option<AsyncNode> {
            &mut self.sibling
        }
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn get_widget(&self) -> &gtk::Widget {
            self.widget.as_ref()
        }

        fn get_widget_as_container(&self) -> &gtk::Container {
            self.widget.as_ref()
        }
    };
}
