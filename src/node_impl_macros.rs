//! macros to implement functions of node trait

/// impl as_any and as_any_mut for node
#[macro_export]
macro_rules! impl_node_trait_as_any {
    () => {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    };
}

/// impl is_dirty, mark_dirty and mark_clean for node
/// need to have is_dirty:bool as a struct field
#[macro_export]
macro_rules! impl_node_trait_dirty {
    () => {
        fn is_dirty(&self) -> bool {
            self.is_dirty
        }
        fn mark_dirty(&mut self) {
            self.is_dirty = true
        }
        fn mark_clean(&mut self) {
            self.is_dirty = false
        }
    };
}

/// impl NodeComponent
#[macro_export]
macro_rules! impl_node_component {
    ($name:ident) => {
        impl NodeComponent for $name {
            impl_node_trait_dirty!();
        }
    };
}

/// impl get_child, get_child_mut, get_parent
#[macro_export]
macro_rules! impl_node_member_getters {
    () => {
        fn get_child(&self) -> &Option<NodeType> {
            &self.child
        }

        fn get_child_mut(&mut self) -> &mut Option<NodeType> {
            &mut self.child
        }

        fn get_parent(&self) -> &WeakNodeType {
            &self.parent
        }
    };
}
