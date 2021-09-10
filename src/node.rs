use crate::{StrongNodeType, WeakNodeType};

/// node of a n-binary tree which can hold both a child and a sibling, with a reference to it's
/// parent
#[derive(Clone)]
pub struct ContainerNode {
    pub parent: WeakNodeType,
    /// some user defined containers need to move their external children to inner node tree
    /// any inner node which volunteers to store external children is a self_substitute
    pub self_substitute: Option<WeakNodeType>,
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}

/// leaf node of a n-binary tree which can hold only one sibling and contains a weak reference to it's
/// parent
#[derive(Clone)]
pub struct WidgetNode {
    pub parent: WeakNodeType,
    pub sibling: Option<StrongNodeType>,
}

/// node of a n-binary tree which can hold a sibling and a child.
#[derive(Clone, Default)]
pub struct TopLevelNode {
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}
