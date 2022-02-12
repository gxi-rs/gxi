use std::rc::Rc;

use gxi::{
    gxi, set_state, Body, ConstContext, IndexedContext, State, Text, VContainer, VNode,
    VNodeContext, VNodeShell, WeakState,
};

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
    std::mem::forget(app());
    //gxi::run(app());
}

fn app() -> VNodeContext {
    let state = State::from(2i32);
    let state2 = State::from(2i32);

    gxi! {
        Body [ //context
            Text ( value = state.borrow().to_string() ), // text
            br,
            button ( on_click = set_state!( *state+=1, [ref state] ) ) [
                Text ( value = "increment" ) 
            ],
    //        if *state == 3 {
    //            button ( on_click = set_state!( state+=1 ) )
    //        },
            div [
                div,
                div
            ]
        ]
    }
}
//
//fn foo() -> VNodeContext {
//    let state = State::from(2i32);
//    let state2 = State::from(2i32);
//
//    let mut __ctx = ConstContext::default();
//
//    let __child = {
//        let mut __node = Body::new();
//        let mut __node = Rc::new(__node);
//
//        let __child = {
//            let mut __node = gxi::Element::from("div");
//            // props
//            // push
//            // if observable props, convert to StrongNode
//            let __node = Rc::new(__node);
//            // observable props
//            {
//                let __node = Rc::downgrade(&__node);
//                state.add_observer(Box::new(move |state| {
//                    if let Some(__node) = __node.upgrade() {
//                        unsafe {
//                            __node.inner_html(state.borrow().to_string());
//                        }
//                        false
//                    } else {
//                        true
//                    }
//                }));
//            }
//
//            __node
//        };
//
//        __node.push(&__node.as_node(), &*__child);
//        __ctx.push(Box::new(__child));
//
//        let __child = {
//            let mut __node = gxi::Element::from("button");
//
//            __node.on_click(set_state!(*state += 1, [ref state]));
//
//            __node
//        };
//
//        __node.push(&__node.as_node(), &*__child);
//        __ctx.push(Box::new(__child));
//
//        // conditional block
//        {
//            let __parent = Rc::downgrade(&__node);
//
//            let __multi_observer = State::from(());
//
//            add_multi_observer(&state, __multi_observer.downgrade());
//            add_multi_observer(&state2, __multi_observer.downgrade());
//
//            let state = state.clone();
//            let _state2 = state2.clone();
//
//            {
//                let mut __ctx = IndexedContext::default();
//                __multi_observer.add_observer(Box::new(move |_| {
//                    if let Some(__parent) = __parent.upgrade() {
//                        if *((**state).borrow()) == 3 {
//                            if __ctx.check_index(1) {
//                                let __child = {
//                                    let mut __node = gxi::Element::from("button");
//                                    __node.on_click(set_state!(*state += 1, [ref state]));
//                                    __node
//                                };
//                                __node.push(&__node.as_node(), &*__child);
//                                __ctx.set_value(Box::from(__child));
//                            }
//                        } else {
//                            __ctx.reset();
//                        }
//                        false
//                    } else {
//                        true
//                    }
//                }));
//            }
//
//            __ctx.push(Box::new(__multi_observer))
//        }
//
//        // notify
//        state.notify();
//        state2.notify();
//        // return
//        __node
//    };
//
//    VNodeContext::from(VNodeShell::Rc(__child), Some(Box::from(__ctx)))
//}

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
