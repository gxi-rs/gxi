use crate::StrongNodeType;

pub fn get_substitute(node: StrongNodeType) -> StrongNodeType {
    let subst_borrow = node.as_ref().borrow();
    if let Ok(node_comp) = subst_borrow.as_component_node() {
        let subst = node_comp.get_self_substitute().as_ref().expect("You are trying to add a child into component which doen't support #children");
        return subst.upgrade().expect(&format!("#children no longer exists. Make sure it lives long enough"));
    }
    drop(subst_borrow);
    node
}
