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

/// impl container trait, should have child field
#[macro_export]
macro_rules! impl_container {
    ($name:ident) => {
        impl Container for $name {
            fn get_child(&self) -> &Option<GxiNodeType> {
                &self.child
            }

            fn get_child_mut(&mut self) -> &mut Option<GxiNodeType> {
                &mut self.child
            }
        }
    };
}

/// impl container trait, should have child field
#[macro_export]
macro_rules! impl_component_node {
    ($name:ident) => {
        impl ComponentNode for $name {
            fn get_self_substitute(&self) -> &Option<WeakGxiNodeType> {
                &self.self_substitute
            }

            fn get_self_substitute_mut(&mut self) -> &mut Option<WeakGxiNodeType> {
                &mut self.self_substitute
            }
        }
    };
}

/// impl get_child, get_child_mut, get_parent
#[macro_export]
macro_rules! impl_node_getters {
    () => {
        fn get_parent(&self) -> &WeakGxiNodeType {
            &self.parent
        }
        fn get_sibling(&self) -> &Option<GxiNodeType> {
            &self.sibling
        }

        fn get_sibling_mut(&mut self) -> &mut Option<GxiNodeType> {
            &mut self.sibling
        }
    };
}
