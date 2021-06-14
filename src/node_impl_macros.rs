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

/// impl as_any and as_any_mut for node
#[macro_export]
macro_rules! impl_node_trait_as_node {
    () => {
        fn as_node(&self) -> &dyn Node {
            self
        }
        fn as_node_mut(&mut self) -> &mut dyn Node {
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
            fn get_child(&self) -> &Option<StrongNodeType> {
                &self.child
            }

            fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
                &mut self.child
            }

            fn as_container(&self) -> &dyn Container {
                self
            }

            fn as_container_mut(&mut self) -> &mut dyn Container {
                self
            }
        }
    };
}

/// impl container trait, should have child field
#[macro_export]
macro_rules! impl_component_node {
    ($name:ident) => {
        impl ComponentNode for $name {
            fn get_self_substitute(&self) -> &Option<WeakNodeType> {
                &self.self_substitute
            }

            fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
                &mut self.self_substitute
            }
        }
    };
    ($name:ident impl_dirty) => {
        impl ComponentNode for $name {
            fn get_self_substitute(&self) -> &Option<WeakNodeType> {
                &self.self_substitute
            }

            fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
                &mut self.self_substitute
            }

            impl_node_trait_dirty!();
        }
    };
}

/// impl get_child, get_child_mut, get_parent
#[macro_export]
macro_rules! impl_node_getters {
    () => {
        fn get_parent(&self) -> &WeakNodeType {
            &self.parent
        }
        fn get_sibling(&self) -> &Option<StrongNodeType> {
            &self.sibling
        }

        fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.sibling
        }
    };
}

#[macro_export]
macro_rules! impl_widget_node_deref {
    ($name:ident $target:path) => {
        impl Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.widget
            }
        }
    };
}
