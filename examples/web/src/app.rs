use std::{cell::RefCell, rc::Rc};

use gxi::{gxi, Body, StrongNodeType, VNode};

pub enum Msg {
    ReRender,
    CustomPanic(&'static str),
}

pub fn App() -> StrongNodeType {
    let value = "hello";

   
    // convert any value moved to gxi! macro, observable depending on the needs
    let value = gxi::Observable::new(value);
    // add this to gxi macro
    let this = Rc::new(RefCell::new(Body::default().into_vnode_type()));
    gxi! {
       h1 [
         button ( on_click = |_| panic!("hello"), inner_html = "click me"),
         p ( inner_html = *value )
       ]
    }
    return __node;
}

//#[render(App)]
//fn render() {
//    gxi! {
//      Body [
//         div [
//            button ( inner_html = "hello buddy", on_click = set_state!(Msg::ReRender) ),
//            hello
//         ],
//         h1 ( inner_html = "23", on_click = set_state!(Msg::CustomPanic("hello")) ) [
//            h1 ( inner_html = "12" ),
//            h1 ( inner_html = "12" )
//         ],
//         crate::Counter,
//         {
//            crate::Counter::render(&__node.clone());
//         }
//      ]
//    }
//}
//
//#[update(App)]
//fn update(msg: Self::Msg) {
//    match msg {
//        Msg::ReRender => (),
//        Msg::CustomPanic(msg) => {
//            panic!("{}", msg)
//        }
//    }
//}
