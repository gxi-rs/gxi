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
    };
}

#[macro_export]
macro_rules! impl_node_trait_get_widget {
    () => {
        fn get_widget(&self) -> &gtk::Widget {
            self.widget.as_ref()
        }

        fn get_widget_as_container(&self) -> &gtk::Container {
            self.widget.as_ref()
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_init_child {
    () => {
        fn init_child(&mut self, f: Box<dyn Fn() -> AsyncNode>) -> (AsyncNode, bool) {
            match self.child {
                None => {
                    let child = self.child.get_or_insert_with(|| f());
                    self.widget.add(child.clone().borrow_mut().get_widget());
                    (child.clone(), true)
                }
                _ => (self.child.as_ref().unwrap().clone(), false),
            }
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_init_sibling {
    () => {
        fn init_sibling(&mut self, f: Box<dyn Fn() -> AsyncNode>) -> (AsyncNode, bool) {
            match self.sibling {
                None => {
                    let sibling = self.sibling.get_or_insert_with(|| f());
                    self.parent
                        .borrow_mut()
                        .get_widget_as_container()
                        .add(sibling.clone().borrow_mut().get_widget());
                    (sibling.clone(), true)
                }
                _ => (self.sibling.as_ref().unwrap().clone(), false),
            }
        }
    };
}
