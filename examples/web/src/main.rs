use std::rc::Rc;

use gxi::{set_state, Body, ConstContextNode, State, VContainer, VNode, WeakState};

//mod app;
//mod cat_fact;
//mod counter;
//mod todo;
//
//pub use cat_fact::*;
//pub use counter::*;
//pub use todo::*;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    app().leak();
    //gxi::run(app());
}

fn app() -> ConstContextNode {
    let state = State::from(2i32);
    let state2 = State::from(2i32);

    {
        let mut __ctx = ConstContextNode::default();
        // rule of thumb __node can only be owned by parent context
        {
            let mut __node = Body::new();
            {
                let mut __parent = Rc::new(__node);

                {
                    let mut __node = gxi::Element::from("div");
                    // props
                    // push
                    __parent.push(&__node.as_node(), &*__node);
                    // if observable props, convert to StrongNode
                    let __node = Rc::new(__node);
                    // observable props
                    {
                        let __node = Rc::downgrade(&__node);
                        state.add_observer(Box::new(move |state| {
                            if let Some(__node) = __node.upgrade() {
                                unsafe {
                                    __node.inner_html(state.borrow().to_string());
                                }
                                false
                            } else {
                                true
                            }
                        }));
                    }
                    // lifetime of this __node ends here
                    // in order to make state observable work
                    // this __node either needs to be forgotten or needs to be put
                    // into someone with higher lifetime
                    __ctx.push(Box::new(__node));
                }

                {
                    let mut __node = gxi::Element::from("button");

                    __node.on_click(set_state!(*state += 1, [ref state]));

                    __parent.push(&__node.as_node(), &*__node);

                    __ctx.push(Box::new(__node));
                }

                // conditional block
                {
                    let mut __if_index = 0;
                    let __parent = Rc::downgrade(&__parent);

                    let __multi_observer = State::from(());

                    add_multi_observer(&state, __multi_observer.downgrade());
                    add_multi_observer(&state2, __multi_observer.downgrade());

                    let state = state.clone();
                    let state2 = state2.clone();

                    __multi_observer.add_observer(Box::new(move |_| {
                        if let Some(__parent) = __parent.upgrade() {
                            if *(**state).borrow() == 3 {
                                if __if_index != 1 {
                                    {
                                        let mut __node = gxi::Element::from("button");

                                        __node.on_click(set_state!(*state += 1, [ref state]));

                                        __parent.push(&__node.as_node(), &*__node);
                                    }
                                    __if_index = 1;
                                }
                            } else {
                            }
                            false
                        } else {
                            true
                        }
                    }));

                    __ctx.push(Box::new(__multi_observer));
                }

                __ctx.push(Box::new(__parent));
            }
        };
        state.notify();
        state2.notify();
        __ctx
    }
}

fn add_multi_observer<V>(state: &State<V>, multi_observer: WeakState<()>) {
    state.add_observer(Box::new(move |_| {
        if let Some(multi_observer) = multi_observer.upgrade() {
            multi_observer.notify();
            false
        } else {
            true
        }
    }));
}
