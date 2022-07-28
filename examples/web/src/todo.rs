use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use gxi::{gxi, set_state, State, Text, VNodeContext};

pub fn todo() -> VNodeContext<gxi::Element> {
    let todos = State::from(String::new());
    let state2 = State::from(String::new());

    {
        use gxi::{VContainer, VLeaf, VNode};
        let mut __ctx = gxi::ConstContext::default();
        let __child = {
            let mut __node = gxi::Element::from("div");
            let __node = std::rc::Rc::new(__node);

            let mut __index_buff = Rc::new(RefCell::new([0usize; 3usize]));

            let __child = {
                let mut __node = gxi::Element::from("input");
                let __node = std::rc::Rc::new(__node);
                __node.on_input({
                    let todos = todos.clone();
                    let state2 = state2.clone();
                    move |e| {
                        {
                            let todos = &mut *(**todos).borrow_mut();
                            let state2 = &mut *(**state2).borrow_mut();
                            {
                                let data = e.data().unwrap_or_default();
                                todos.push_str(&data);
                                state2.push_str(&data);
                            }
                        }
                        todos.notify();
                        state2.notify();
                    }
                });
                __node
            };
            __node.push(&__child.as_node(), &*__child);
            __ctx.push(Box::new(__child));

            let __child = {
                let mut __node = gxi::Element::from("div");
                let __child = {
                    let mut __node = Text::new();
                    __node.value("3rd element");
                    __node
                };
                __node.push(&__child.as_node(), &*__child);
                __node
            };
            __node.push(&__child.as_node(), &*__child);

            let __child = {
                let mut __node = Text::new();
                let __node = std::rc::Rc::new(__node);
                {
                    let __node = std::rc::Rc::downgrade(&__node);
                    todos.add_observer(Box::new(move |todos| {
                        if let Some(__node) = __node.upgrade() {
                            let todos = todos.borrow();
                            __node.value(&*todos);
                            false
                        } else {
                            true
                        }
                    }));
                }
                __node
            };
            __node.push(&__child.as_node(), &*__child);
            __ctx.push(Box::new(__child));

            __ctx.push(Box::new({
                let __node = std::rc::Rc::downgrade(&__node);
                // FIX: fix this in transpiler
                let mut __ctx = gxi::IndexedContext::default();
                let __ctx = std::rc::Rc::new(std::cell::RefCell::new(__ctx));

                {
                    let __ctx = std::rc::Rc::downgrade(&__ctx);
                    todos.add_observer(Box::new(move |todos| {
                        // FIX: add this as well
                        if let (Some(__node), Some(mut __ctx)) = (__node.upgrade(), __ctx.upgrade())
                        {
                            use std::ops::{Deref, DerefMut};
                            if {
                                let todos = todos.borrow();
                                *todos == "abc" || *todos == "abcde"
                            } {
                                if (*__ctx).borrow_mut().check_index(1usize) {
                                    let (index, range_to_remove, should_replace) =
                                        gxi::IndexedContext::compute_index_helper(
                                            &mut *(*__index_buff).borrow_mut(),
                                            0,
                                            3,
                                            3,
                                        );

                                    let __child = {
                                        let mut __node = gxi::Element::from("div");
                                        unsafe {
                                            __node.inner_html("abcd");
                                        }
                                        __node
                                    };
                                    __node.insert_at_index(
                                        &__child.as_node(),
                                        &*__child,
                                        index + 0,
                                        should_replace,
                                    );
                                    let __child = {
                                        let mut __node = gxi::Element::from("div");
                                        unsafe {
                                            __node.inner_html("_foo");
                                        }
                                        __node
                                    };
                                    __node.insert_at_index(
                                        &__child.as_node(),
                                        &*__child,
                                        index + 1,
                                        should_replace,
                                    );
                                    let __child = {
                                        let mut __node = gxi::Element::from("div");
                                        unsafe {
                                            __node.inner_html("_asdf");
                                        }
                                        __node
                                    };
                                    __node.insert_at_index(
                                        &__child.as_node(),
                                        &*__child,
                                        index + 2,
                                        should_replace,
                                    );

                                    __node.remove_elements(range_to_remove);
                                }
                            } else {
                                if (*__ctx).borrow_mut().check_index(0usize) {
                                    let (index, range_to_remove, should_replace) =
                                        gxi::IndexedContext::compute_index_helper(
                                            &mut *(*__index_buff).borrow_mut(),
                                            0,
                                            0,
                                            3,
                                        );

                                    __node.remove_elements(range_to_remove);
                                }
                            }
                            false
                        } else {
                            true
                        }
                    }));
                }

                __ctx
            }));

            let __child = {
                let mut __node = gxi::Element::from("div");
                let __child = {
                    let mut __node = Text::new();
                    __node.value("4th element");
                    __node
                };
                __node.push(&__child.as_node(), &*__child);
                __node
            };

            __node.push(&__child.as_node(), &*__child);
            {
                let __node = std::rc::Rc::downgrade(&__node);
                let mut __ctx = gxi::IndexedContext::default();
                todos.add_observer(Box::new(move |todos| {
                    if let Some(__node) = __node.upgrade() {
                        use std::ops::{Deref, DerefMut};
                        if {
                            let todos = todos.borrow();
                            *todos == "abcdefgh"
                        } {
                            if __ctx.check_index(1usize) {
                                let __child = {
                                    let mut __node = Text::new();
                                    __node.value("*_*");
                                    __node
                                };
                                __node.push(&__child.as_node(), &*__child);
                            }
                        }
                        false
                    } else {
                        true
                    }
                }));
            }
            {
                let __node = std::rc::Rc::downgrade(&__node);
                let mut __ctx = gxi::IndexedContext::default();
                todos.add_observer(Box::new(move |todos| {
                    if let Some(__node) = __node.upgrade() {
                        use std::ops::{Deref, DerefMut};
                        if {
                            let todos = todos.borrow();
                            *todos == "abcdefghi"
                        } {
                            if __ctx.check_index(1usize) {
                                let __child = {
                                    let mut __node = Text::new();
                                    __node.value("yay");
                                    __node
                                };
                                __node.push(&__child.as_node(), &*__child);
                            }
                        }
                        false
                    } else {
                        true
                    }
                }));
            }
            let __child = {
                let mut __node = gxi::Element::from("div");
                let __child = {
                    let mut __node = Text::new();
                    __node.value("5th element");
                    __node
                };
                __node.push(&__child.as_node(), &*__child);
                __node
            };
            __node.push(&__child.as_node(), &*__child);
            __node
        };
        gxi::VNodeContext::WithCtx(gxi::VNodeShell::Rc(__child), Box::from(__ctx))
    }
    //    gxi! {
    //        div [
    //            input ( on_input = set_state!(|e| {
    //                let data = e.data().unwrap_or_default();
    //                todos.push_str(&data);
    //                state2.push_str(&data);
    //            }, [ref todos,ref state2]) ),
    //            div [
    //                Text ( value = "3rd element" )
    //            ],
    //            Text ( value = &*todos ),
    //            if *todos == "abc" {
    //                {
    //                    panic!("abc");
    //                },
    //                Text ( value = "abcd" ),
    //                Text ( value = "_foo")
    //            },
    //            div [
    //                Text ( value = "4th element" )
    //            ],
    //            if *todos == "abcdefgh" {
    //                Text ( value = "*_*")
    //            },
    //            if *todos == "abcdefghi" {
    //                Text ( value = "yay" )
    //            },
    //            div [
    //                Text ( value = "5th element" )
    //            ]
    //        ]
    //    }
}
