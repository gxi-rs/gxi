use crate::{InitType, WeakNodeType, StrongNodeType};

// TODO: replace init_type with f32 index
/// if init_type doesn't already exist then run init() and return clone of the new member
///
/// @return
/// + bool: false if child already exists
pub fn init_member<F: FnOnce(WeakNodeType) -> StrongNodeType>(
    mut this: StrongNodeType, init_type: InitType, init: F,
) -> (StrongNodeType, bool) {
    /*if let GxiNodeType::Widget(_) = &this {
        panic!("Can't add a node into a widget");
    }*/
    todo!()
    /*match init_type {
        InitType::Child => {
            let this_node = this.clone().into_gxi_container_rc().unwrap();
            // check if child already exists
            {
                // scope to drop widget_node to prevent ownership errors
                let this_node = this_node.as_ref().borrow_mut();
                if let Some(child) = this_node.get_child() {
                    return (child.clone(), false);
                }
            }
            // if child does not exist initialize it
            let child = init(this.downgrade());
            // if child is a widget or a container add it's widget to this if this is also a widget
            if let Ok(child) = child.clone().into_gxi_widget_rc() {
                let child_borrow = child.borrow();
                match &mut this {
                    GxiNodeType::Widget(this) => {
                        let mut this_borrow = this.as_ref().borrow_mut();
                        this_borrow.get_widget_mut().append(child_borrow.get_widget());
                    }
                    GxiNodeType::Component(_this) => {
                        // while parent isn't widget
                        // TODO: implement this
                        /*loop {
                            let mut this_borrow = this.borrow_mut();
                            let parent = this_borrow.get_parent().clone().upgrade();
                            let parent = parent.into_gxi_node_rc();
                        }*/
                    }
                    _ => unreachable!()
                }
            }

            let mut this_node_borrow = this_node.as_ref().borrow_mut();
            *this_node_borrow.get_child_mut() = Some(child.clone());
            return (child, true);
        }
        _ => {
            todo!()
        }
    }*/
}
