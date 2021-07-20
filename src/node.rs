use crate::StrongNodeType;

/// node of a n-binary tree which can hold a child and a sibling
#[derive(Clone, Default)]
pub struct ContainerNode {
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}

/// node of a n-binary tree which can hold only a sibling
#[derive(Clone, Default)]
pub struct WidgetNode {
    pub sibling: Option<StrongNodeType>,
}

