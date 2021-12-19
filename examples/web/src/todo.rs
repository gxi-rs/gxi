use gxi::{gxi, set_state, State, StrongNodeType, Text};

pub fn todo() -> StrongNodeType {
    let todos = State::new(String::new());

    let state2 = State::new(String::new());

    return gxi! {
        div [
            input ( on_input = set_state!(|e| {
                let data = e.data().unwrap_or(String::new());
                todos.push_str(&data);
                state2.push_str(&data);
            }, [ref todos,ref state2]) ),
            if *todos == "a" {
                div [ Text ( value = "hi" ) ]
            } else if const *todos == "ab" {
                div [ Text ( value = "hi brother" ) ]
            } else if const *todos == "abcd"{
                Text ( value = "z" )  
            } else {
                div [
                    Text ( value = "none" ),
                    if *state2 == "abcde" || *state2 == "abcdefg" {
                        div [
                            Text ( value = "zzzzzzzzzz" )
                        ]
                    }
                ]
            },
            div [
                Text ( value = "3rd element" )
            ],
            if *todos == "abc" {
                Text ( value = "abcd" )
            },
            div [
                Text ( value = "4th element" )
            ],
            div [
                Text ( value = "5th element" )
            ],
        ]
    };
}
//return {
//    use gxi::{VContainerWidget, VNode};
//    let mut __node = gxi::Element::from_str("div");
//    __node.push(gxi! {
//        input ( on_input = set_state!(|e| {
//            let data = e.data().unwrap_or(String::new());
//            todos.push_str(&data);
//        }, [ref todos]) )
//    });
//    let __node = __node.into_strong_node_type();
//    {
//        let __node = std::rc::Rc::downgrade(&__node);
//        let __if_counter = State::new(0usize);
//        todos.add_observer(Box::new(move |todos| {
//            const __INDEX: usize = 1;

//            use std::ops::DerefMut;
//            if let Some(__node) = __node.upgrade() {
//                let mut __node = __node.as_ref().borrow_mut();
//                let mut __node = __node
//                    .deref_mut()
//                    .as_mut()
//                    .downcast_mut::<gxi::Element>()
//                    .unwrap();
//                let mut __if_counter = __if_counter.deref().borrow_mut();
//                // add logic goes here
//                if todos == "hello" {
//                    if *__if_counter != 1 {
//                        __node.set_at_index(
//                            Some(gxi! {
//                                Text ( value = "hi" )
//                            }),
//                            __INDEX,
//                            // pu
//                            *__if_counter == 0 || *__if_counter == 3,
//                        );
//                        *__if_counter = 1;
//                    }
//                } else if todos == "hello friend" {
//                    if *__if_counter != 2 {
//                        __node.set_at_index(
//                            Some(gxi! {
//                                Text ( value = "hi brother" )
//                            }),
//                            __INDEX,
//                            *__if_counter == 0 || *__if_counter == 3,
//                        );
//                        *__if_counter = 2;
//                    }
//                } else {
//                    if *__if_counter != 3 {
//                        __node.set_at_index(
//                            None,
//                            __INDEX,
//                            *__if_counter == 0 || *__if_counter == 3,
//                        );
//                        *__if_counter = 3;
//                    }
//                }
//                false
//            } else {
//                true
//            }
//        }));
//    }

//    // add other elements
//    {
//        use std::ops::DerefMut;

//        let mut __node = __node.as_ref().borrow_mut();
//        let mut __node = __node
//            .deref_mut()
//            .as_mut()
//            .downcast_mut::<gxi::Element>()
//            .unwrap();

//        __node.push(gxi! {
//            div [
//                br,
//                Text ( value = "3rd element" )
//            ]
//        });
//    }

//    // any other if block can go here
//    __node
//};
