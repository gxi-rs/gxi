use crate::VNodeType;

/// node of a n-binary tree which can hold a child and a sibling
#[derive(Default)]
pub struct Node {
    pub child: Option<VNodeType>,
    pub sibling: Option<VNodeType>,
}
