use crate::{StrongNodeType, WeakNodeType};

/// node of a n-binary tree which can hold a child and a sibling
#[derive(Clone, Default)]
pub struct ContainerNode {
    pub parent: WeakNodeType,
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}

/// node of a n-binary tree which can hold only a sibling
#[derive(Clone, Default)]
pub struct WidgetNode {
    pub parent: WeakNodeType,
    pub sibling: Option<StrongNodeType>,
}

#[derive(Clone, Default)]
pub struct TopLevelNode {
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}
