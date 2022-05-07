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

            let mut index_buff = [0; 10];

            let __child = {
                let mut __node = gxi::Element::from("input");
                __node.on_input({
                    let todos = todos.clone();
                    let state2 = state2.clone();
                    move |e| {
                        {
                            let todos = &mut *(**todos).borrow_mut();
                            let state2 = &mut *(**state2).borrow_mut();
                            {
                                let data = e.data().unwrap_or(String::new());
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

            {
                let __node = std::rc::Rc::downgrade(&__node);
                let mut __ctx = gxi::IndexedContext::default();
                todos.add_observer(Box::new(move |todos| {
                    if let Some(__node) = __node.upgrade() {
                        let constant_nodes_prior = 1;

                        use std::ops::{Deref, DerefMut};
                        if {
                            let todos = todos.borrow();
                            *todos == "a"
                        } {
                            if __ctx.check_index(1usize) {
                                let __child = {
                                    let mut __node = gxi::Element::from("div");
                                    let __child = {
                                        let mut __node = Text::new();
                                        __node.value("hi");
                                        __node
                                    };
                                    __node.push(&__child.as_node(), &*__child);
                                    __node
                                };
                                __node.set_at_index(&__child.as_node(), 1usize);
                            }
                        } else if {
                            let todos = todos.borrow();
                            *todos == "ab"
                        } {
                            if __ctx.check_index(2usize) {
                                let __child = {
                                    let mut __node = gxi::Element::from("div");
                                    let __child = {
                                        let mut __node = Text::new();
                                        __node.value("hi brother");
                                        __node
                                    };
                                    __node.push(&__child.as_node(), &*__child);
                                    __node
                                };
                                __node.set_at_index(&__child.as_node(), 1usize);
                            }
                        } else if {
                            let todos = todos.borrow();
                            *todos == "abcd"
                        } {
                            if __ctx.check_index(3usize) {
                                let __child = {
                                    let mut __node = Text::new();
                                    __node.value("z");
                                    __node
                                };
                                __node.set_at_index(&__child.as_node(), 1usize);
                            }
                        } else if __ctx.check_index(4usize) {
                            let __child = {
                                let mut __node = gxi::Element::from("div");
                                let __node = std::rc::Rc::new(__node);
                                let mut __extra_nodes_counter = 0usize;
                                let __child = {
                                    let mut __node = Text::new();
                                    __node.value("none");
                                    __node
                                };
                                __node.push(&__child.as_node(), &*__child);
                                {
                                    let __node = std::rc::Rc::downgrade(&__node);
                                    let mut __ctx = gxi::IndexedContext::default();
                                    state2.add_observer(Box::new(move |state2| {
                                        if let Some(__node) = __node.upgrade() {
                                            use std::ops::{Deref, DerefMut};
                                            if {
                                                let state2 = state2.borrow();
                                                *state2 == "abcde" || *state2 == "abcdefg"
                                            } {
                                                if __ctx.check_index(1usize) {
                                                    let __child = {
                                                        let mut __node = gxi::Element::from("div");
                                                        let __child = {
                                                            let mut __node = Text::new();
                                                            __node.value("zzzzzzzzzz");
                                                            __node
                                                        };
                                                        __node.push(&__child.as_node(), &*__child);
                                                        __node
                                                    };
                                                    __node.set_at_index(&__child.as_node(), 1usize);
                                                }
                                            } else {
                                                if __ctx.check_index(2usize) {}
                                            }
                                            false
                                        } else {
                                            true
                                        }
                                    }));
                                }
                                __node
                            };
                            __node.set_at_index(&__child.as_node(), 1usize);
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
                    __node.value("3rd element");
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
                            *todos == "abc"
                        } {
                            if __ctx.check_index(1usize) {
                                let __child = {
                                    let mut __node = Text::new();
                                    __node.value("abcd");
                                    __node
                                };
                                __node.set_at_index(&__child.as_node(), 2usize);
                                let __child = {
                                    let mut __node = Text::new();
                                    __node.value("_foo");
                                    __node
                                };
                                __node.set_at_index(&__child.as_node(), 3usize);
                            }
                        } else {
                            if __ctx.check_index(2usize) {}
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
                                __node.set_at_index(&__child.as_node(), 3usize);
                            }
                        } else {
                            if __ctx.check_index(2usize) {}
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
                                __node.set_at_index(&__child.as_node(), 3usize);
                            }
                        } else {
                            if __ctx.check_index(2usize) {}
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
        gxi::VNodeContext::WithCtx(gxi::VNodeShell::Default(__child), Box::from(__ctx))
    }

    //    gxi! {
    //        div [
    //            input ( on_input = set_state!(|e| {
    //                let data = e.data().unwrap_or(String::new());
    //                todos.push_str(&data);
    //                state2.push_str(&data);
    //            }, [ref todos,ref state2]) ),
    //            if *todos == "a" {
    //                div [ Text ( value = "hi" ) ]
    //            } else if *todos == "ab" {
    //                div [ Text ( value = "hi brother" ) ]
    //            } else if *todos == "abcd"{
    //                Text ( value = "z" )
    //            } else {
    //                div [
    //                    Text ( value = "none" ),
    //                    if *state2 == "abcde" || *state2 == "abcdefg" {
    //                        div [
    //                            Text ( value = "zzzzzzzzzz" )
    //                        ]
    //                    }
    //                ]
    //            },
    //            div [
    //                Text ( value = "3rd element" )
    //            ],
    //            if *todos == "abc" {
    //                Text ( value = "abcd" ),
    //                Text ( value = "_foo")
    //            },
    //            div [
    //                Text ( value = "4th element" )
    //            ],
    //            if *todos == "abcdefgh" {
    //                    Text ( value = "*_*")
    //            },
    //            if *todos == "abcdefghi" {
    //                Text ( value = "yay" )
    //            },
    //            div [
    //                Text ( value = "5th element" )
    //            ],
    //        ]
    //    }
}
