use crate::{self as gxi, TopLevelNode};

#[derive(Default,gxi::TopLevelContainer)]
pub struct Root {
    node: TopLevelNode
}
