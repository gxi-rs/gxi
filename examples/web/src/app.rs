use std::rc::Rc;

use gxi::{gxi, init_member, InitType, StrongNodeType, VNode};
use gxi::{update, Body, WeakNodeType};

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

enum Msg {
    ReRender,
    CustomPanic(&'static str),
}

impl gxi::Renderable for App {
    fn render(this: &StrongNodeType) {
        let __node = this.clone();

        gxi! {
             Body [
                 div [
                     button ( inner_html = "hello buddy", on_click = update!(Msg::ReRender) ),
                     hello
                 ],
                 h1 ( inner_html = "23", on_click = update!(Msg::CustomPanic("hello")) ) [
                     h1 ( inner_html = "12" ),
                     h1 ( inner_html = "12" )
                 ]
             ]
        }
    }
}

impl App {
    fn update(this: &WeakNodeType, msg: Msg) {
        use gxi::Renderable;
        match msg {
            Msg::ReRender => Self::render(&this.upgrade().unwrap()),
            Msg::CustomPanic(msg) => {
                panic!("{}", msg)
            }
        }
    }
}
