#[macro_export]
macro_rules! impl_node_trait {
    () => {
        fn is_dirty(&self) -> bool {
            self.dirty.clone()
        }
        fn mark_dirty(&mut self) {
            self.dirty = true;
        }
        fn mark_clean(&mut self) {
            self.dirty = false;
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
macro_rules! impl_node_trait_init_sibling {
    () => {
        fn init_sibling(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
            match self.sibling {
                None => {
                    let sibling = self.sibling.get_or_insert(f());
                    if let NodeType::Widget = sibling.as_ref().borrow().get_type() {
                        let sibling_borrow = sibling.as_ref().borrow();
                        // parent_container.add(&sibling_borrow.get_widget());
                    }
                    (sibling.clone(), true)
                }
                _ => (self.sibling.as_ref().unwrap().clone(), false),
            }
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_init_child {
    () => {
        fn init_child(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
            match self.child {
                None => {
                    let child = self.child.get_or_insert(f());
                    if let NodeType::Widget = child.as_ref().borrow().get_type() {
                        let child_borrow = child.as_ref().borrow();
                        self.widget.add(&child_borrow.get_widget());
                    }
                    (child.clone(), true)
                }
                _ => (self.child.as_ref().unwrap().clone(), false),
            }
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_get_child {
    () => {
        fn get_child(&self) -> &Option<NodeRc> {
            &self.child
        }

        fn get_child_mut(&mut self) -> &mut Option<NodeRc> {
            &mut self.child
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_get_sibling {
    () => {
        fn get_sibling(&self) -> &Option<NodeRc> {
            &self.sibling
        }

        fn get_sibling_mut(&mut self) -> &mut Option<NodeRc> {
            &mut self.sibling
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
    };
}

#[macro_export]
macro_rules! impl_node_trait_get_widget_as_container {
    () => {
        fn get_widget_as_container(&self) -> gtk::Container {
            let widget: &gtk::Container = self.widget.as_ref();
            widget.clone()
        }
    };
}
#[macro_export]
macro_rules! impl_node_for_component {
    () => {
        impl_node_trait!();
        impl_node_trait_get_child!();
        impl_node_trait_get_sibling!();
        impl_node_trait_init_sibling!();

        fn get_widget(&self) -> gtk::Widget {
            let parent = self.parent.upgrade().unwrap();
            let parent = parent.as_ref().borrow();
            parent.get_widget()
        }

        fn get_widget_as_container(&self) -> gtk::Container {
            let parent = self.parent.upgrade().unwrap();
            let parent = parent.as_ref().borrow();
            parent.get_widget_as_container()
        }

        fn init_child(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
            match self.child {
                None => {
                    let child = self.child.get_or_insert(f());
                    if let NodeType::Widget = child.as_ref().borrow().get_type() {
                        let child_borrow = child.as_ref().borrow();
                        let parent = self.parent.upgrade().unwrap();
                        let parent = parent.as_ref().borrow_mut();
                        parent.get_widget_as_container().add(&child_borrow.get_widget());
                    }
                    (child.clone(), true)
                }
                _ => (self.child.as_ref().unwrap().clone(), false),
            }
        }

        fn get_type(&self) -> NodeType {
            NodeType::Component
        }
    };
}

#[macro_export]
macro_rules! impl_drop_for_node {
    ($ident:ident) => {
        impl Drop for $ident {
            fn drop(&mut self) {
                unsafe {
                    self.widget.destroy();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_drop_for_component {
    ($ident:ident) => {
        impl Drop for $ident {
            fn drop(&mut self) {
                // Components need to not drop anything
            }
        }
    };
}
