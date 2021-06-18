use gxi::*;

pub fn no_sibling(node: StrongNodeType) {
    let node_borrow = node.as_ref().borrow();
    if node_borrow.as_node().get_sibling().is_some() {
        panic!("no sibling was expected");
    }
}

pub fn check_child_type<T: 'static + Node>(node: StrongNodeType, name: &str) -> StrongNodeType {
    let node_borrow = node.as_ref().borrow();
    let child = node_borrow
        .as_container()
        .unwrap()
        .get_child()
        .as_ref()
        .unwrap()
        .clone();
    let child_borrow = child.as_ref().borrow();
    child_borrow
        .as_node()
        .as_any()
        .downcast_ref::<T>()
        .expect(&format!("expected '{}' here", name));
    drop(child_borrow);
    child
}

pub fn check_substs_child_type<T: 'static + Node>(
    node: StrongNodeType,
    name: &str,
) -> StrongNodeType {
    let node_borrow = node.as_ref().borrow();
    let subst = node_borrow
        .as_component_node()
        .unwrap()
        .get_self_substitute()
        .as_ref()
        .unwrap()
        .upgrade()
        .unwrap();
    let subst_borrow = subst.as_ref().borrow();
    let subst_child = subst_borrow
        .as_container()
        .unwrap()
        .get_child()
        .as_ref()
        .unwrap()
        .clone();
    subst_child
        .as_ref()
        .borrow()
        .as_node()
        .as_any()
        .downcast_ref::<T>()
        .expect(&format!("expected '{}' here", name));
    drop(subst_borrow);
    subst_child
}

pub fn check_sibling_type<T: 'static + Node>(node: StrongNodeType, name: &str) -> StrongNodeType {
    let node_borrow = node.as_ref().borrow();
    let sibling = node_borrow
        .as_node()
        .get_sibling()
        .as_ref()
        .unwrap()
        .clone();
    let sibling_borrow = sibling.as_ref().borrow();
    sibling_borrow
        .as_node()
        .as_any()
        .downcast_ref::<T>()
        .expect(&format!("expected '{}' here", name));
    drop(sibling_borrow);
    sibling
}
