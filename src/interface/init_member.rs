use crate::{NodeType, InitType, WeakGxiNodeType, GxiNodeRc};

// TODO: replace init_type with f32 index
/// if init_type doesn't already exist then run init() and return clone of the new member
///
/// @return
/// + bool: false if child already exists
pub fn init_member<F: FnOnce(WeakGxiNodeType) -> NodeType>(
    this: NodeType,
    init_type: InitType,
    init: F,
) -> (NodeType, bool) {
    match init_type {
        InitType::Child => {
            let this_node: GxiNodeRc = this.clone().into_gxi_node_rc();
            {
                // scope to drop widget_node to prevent ownership errors
                let this_node = this_node.as_ref().borrow_mut();
                if let Some(child) = this_node.get_child() {
                    return (child.clone(), false);
                }
            }
            // if child does not exist initialize it
            let child = init(this.clone().downgrade());
            // if child is a widget add it's widget to this if this is also a widget
            if let NodeType::Widget(child) = &child {
                let child_borrow = child.borrow();
                match &this {
                    NodeType::Widget(this) => {
                        let mut this_borrow = this.borrow_mut();
                        this_borrow
                            .get_widget_mut()
                            .append(child_borrow.get_widget());
                    }
                    NodeType::Component(_this) => {
                        // while parent isn't widget
                        // TODO: implement this
                        /*loop {
                            let mut this_borrow = this.borrow_mut();
                            let parent = this_borrow.get_parent().clone().upgrade();
                            let parent = parent.into_gxi_node_rc();
                        }*/
                    }
                }
            }

            let mut this_node_borrow = this_node.as_ref().borrow_mut();
            let child = this_node_borrow.get_child_mut().replace(child).unwrap();
            return (child, true);
        }
        _ => {
            todo!()
        }
    }
}
