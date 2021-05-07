#[macro_export]
macro_rules! impl_add_for_desktop_node {
    () => {
        fn add(&mut self, child: NodeRc) {
            self.widget.add(&child.as_ref().borrow().get_widget());
            self.mark_dirty();
        }
    };
}
