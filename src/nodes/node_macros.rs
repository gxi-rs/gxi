#[macro_export]
macro_rules! impl_node_trait {
    () => {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_get_widget {
    () => {
        fn get_widget(&self) -> gtk::Widget {
            let widget: &gtk::Widget = self.widget.as_ref();
            widget.clone()
        }

        fn get_widget_as_container(&self) -> gtk::Container {
            let widget: &gtk::Container = self.widget.as_ref();
            widget.clone()
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_init_sibling {
    () => {
        fn init_sibling(
            &mut self, f: Box<dyn Fn() -> AsyncNode>, add_widget: bool,
        ) -> (AsyncNode, bool) {
            match self.sibling {
                None => {
                    let sibling = self.sibling.get_or_insert_with(|| f());
                    if add_widget {
                        let parent_borrow = self.parent.as_ref().borrow();
                        let parent_container = parent_borrow.get_widget_as_container();
                        let sibling_borrow = sibling.as_ref().borrow();
                        parent_container.add(&sibling_borrow.get_widget());
                    }
                    (sibling.clone(), true)
                }
                _ => (self.sibling.as_ref().unwrap().clone(), false),
            }
        }
        fn get_sibling(&self) -> &Option<AsyncNode> {
            &self.sibling
        }

        fn get_sibling_mut(&mut self) -> &mut Option<AsyncNode> {
            &mut self.sibling
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_init_child {
    () => {
        fn init_child(
            &mut self, f: Box<dyn Fn() -> AsyncNode>, add_widget: bool,
        ) -> (AsyncNode, bool) {
            match self.child {
                None => {
                    let child = self.child.get_or_insert_with(|| f());
                    if add_widget {
                        let child_borrow = child.as_ref().borrow();
                        self.widget.add(&child_borrow.get_widget());
                    }
                    (child.clone(), true)
                }
                _ => (self.child.as_ref().unwrap().clone(), false),
            }
        }
        impl_node_trait_get_child!();
    };
}

#[macro_export]
macro_rules! impl_node_trait_get_child {
    () => {
        fn get_child(&self) -> &Option<AsyncNode> {
            &self.child
        }

        fn get_child_mut(&mut self) -> &mut Option<AsyncNode> {
            &mut self.child
        }
    };
}
