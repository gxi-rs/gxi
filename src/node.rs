use crate::StrongNodeType;

/// node of a n-binary tree which can hold both a child and a sibling, with a reference to it's
/// parent
#[derive(Default)]
pub struct TreeNode {
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}

#[derive(Default)]
pub struct TreeLeafNode {
    pub sibling: Option<StrongNodeType>,
}
