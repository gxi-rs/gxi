use crate::StrongNodeType;

pub fn get_substitute(node: StrongNodeType) -> StrongNodeType {
    let subst_borrow = node.as_ref().borrow();
    if let Ok(node_comp) = subst_borrow.as_component_node() {
        if let Some(subst) = node_comp.get_self_substitute() {
            return subst
                .upgrade()
                .expect(&format!("#children no longer exists. Make sure it lives long enough"));
        }
    }
    drop(subst_borrow);
    node
}