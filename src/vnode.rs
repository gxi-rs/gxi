use std::ops::{Deref, DerefMut, Range};

use crate::{NativeContainer, NativeWidget, Node};

/// Smallest node which can be added to other nodes but
/// it itself may or may not have the ability to hold a child
pub trait VNode {
    /// create a new instance of the node
    /// > TopLevelNode doesn't require a parent yet it needs to implement this function
    /// > to maintain a common interface for the gxi compiler
    fn new() -> Self
    where
        Self: Sized;

    fn as_node(&self) -> Node;
}

/// VNode referring to a native widget. It itself can't hold other widgets
pub trait VLeaf: VNode + Deref<Target = NativeWidget> + DerefMut {}

/// VNode referring to a native widget which can hold other widgets
pub trait VContainer: VNode + Deref<Target = NativeContainer> + DerefMut {
    #[allow(unused_variables)]
    #[allow(clippy::needless_return)]
    fn push(&self, member: &Node, native_widget: &NativeWidget) {
        // do not add widget of to top level container widget
        if let Node::TopLevelContainer = member {
            return;
        }

        #[cfg(feature = "web")]
        self.append_child(native_widget).unwrap();
    }

    #[allow(unused_variables)]
    #[allow(clippy::needless_return)]
    fn insert_at_index(
        &self,
        member: &Node,
        native_widget: &NativeWidget,
        index: usize,
        should_replace: bool,
    ) {
        if let Node::TopLevelContainer = member {
            return;
        }

        #[cfg(feature = "web")]
        {
            let index = index as u32;
            let old = self.children().item(index);
            if should_replace {
                self.replace_child(native_widget, &old.unwrap()).unwrap();
            } else if let Some(old) = self.children().item(index) {
                self.insert_before(native_widget, Some(&old)).unwrap();
            } else {
                self.append_child(native_widget).unwrap();
            }
        }
    }

    #[allow(unused_variables)]
    fn remove_elements(&self, range: Range<usize>) {
        #[cfg(feature = "web")]
        if !range.is_empty() {
            let index = range.start as u32;
            for _ in range {
                self.children().item(index).unwrap().remove();
            }
        }
    }
}
