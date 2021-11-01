use gxi::{gxi, set_state, State, StrongNodeType, Text};

pub fn todo() -> StrongNodeType {
    let todos = State::new(String::new());

    return {
        use gxi::{VContainerWidget, VNode};
        let mut __node = gxi::Element::from_str("div");
        __node.push({
            let mut __node = gxi::Element::from_str("input");
            __node.on_input({
                let todos = todos.clone();
                move |e| {
                    {
                        let todos = &mut *(*todos).borrow_mut();
                        {
                            let data = e.data().unwrap_or(String::new());
                            todos.push_str(&data);
                        }
                    }
                    todos.notify();
                }
            });
            let __node = __node.into_strong_node_type();
            __node
        });
        let __node = __node.into_strong_node_type();
        {
            let __node = std::rc::Rc::downgrade(&__node);
            todos.add_observer(Box::new(move |todos| {
                use std::ops::DerefMut;
                if let Some(__node) = __node.upgrade() {
                    let mut __node = __node.as_ref().borrow_mut();
                    let __node = __node
                        .deref_mut()
                        .as_mut()
                        .downcast_mut::<gxi::Element>()
                        .unwrap();
                    // add logic goes here
                    if todos == "hello" {
                        __node.push({
                            gxi! {
                                Text ( value = "hi" )
                            }
                        })
                    } else {
                        __node.push({
                            gxi! {
                                Text ( value = "1" )
                            }
                        })
                    }
                    false
                } else {
                    true
                }
            }));
        }


        __node
    };
}
