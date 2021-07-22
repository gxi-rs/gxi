use crate::{self as gxi, ContainerNode, TopLevelNode};

#[derive(Default, gxi::TopLevelContainerWidget)]
struct Root {
    node: TopLevelNode,
    native_widget: ()
}


