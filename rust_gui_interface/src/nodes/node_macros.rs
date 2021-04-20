#[macro_export]
macro_rules! impl_node_dirty {
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
    };
}

#[macro_export]
macro_rules! impl_node_as_any {
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
macro_rules! impl_node_trait_init_sibling {
    () => {
        fn init_sibling(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
            match self.sibling {
                None => {
                    let sibling = self.sibling.get_or_insert(f());
                    if let NodeType::Widget = sibling.as_ref().borrow().get_type() {
                        let parent = self.parent.upgrade().unwrap();
                        parent.as_ref().borrow_mut().add(sibling.clone());
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
                    let child = self.child.get_or_insert(f()).clone();
                    if let NodeType::Widget = child.as_ref().borrow().get_type() {
                        self.add(child.clone());
                    }
                    (child, true)
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
        fn get_widget(&self) -> NativeWidget {
            let widget: &NativeWidget = self.widget.as_ref();
            widget.clone()
        }
    };
}

#[macro_export]
macro_rules! impl_node_trait_add {
    () => {
        fn add(&mut self, child: NodeRc) {
            self.widget.add(&child.as_ref().borrow().get_widget());
            self.mark_dirty();
        }
    };
}

#[macro_export]
macro_rules! impl_node_for_component {
    () => {
        impl_node_as_any!();
        impl_node_dirty!();
        impl_node_trait_get_child!();
        impl_node_trait_get_sibling!();
        impl_node_trait_init_sibling!();
        impl_node_trait_substitute!();

        fn add(&mut self, child: NodeRc) {
            let parent = self.parent.upgrade().unwrap();
            parent.as_ref().borrow_mut().add(child);
        }

        fn get_widget(&self) -> NativeWidget {
            let parent = self.parent.upgrade().unwrap();
            let parent = parent.as_ref().borrow();
            parent.get_widget()
        }

        fn init_child(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
            match self.child {
                None => {
                    let child = self.child.get_or_insert(f());
                    if let NodeType::Widget = child.as_ref().borrow().get_type() {
                        let parent = self.parent.upgrade().unwrap();
                        parent.as_ref().borrow_mut().add(child.clone());
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

#[macro_export]
macro_rules! impl_node_trait_substitute {
    () => {
        fn get_self_substitute(&self) -> NodeRc {
            let prost = self.self_substitute.as_ref().unwrap();
            prost.upgrade().unwrap()
        }

        fn set_self_substitute(&mut self, self_substitute: NodeRc) {
            self.self_substitute = Some(Rc::downgrade(&self_substitute));
        }
    };
}
