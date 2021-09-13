use std::rc::Rc;

use gxi::{gxi, init_member, InitType, StrongNodeType, VNode};
use gxi::{Body, WeakNodeType};

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

enum Msg {
    Panic,
}

impl gxi::Renderable for App {
    fn render(this: &StrongNodeType) {
        let __node = this.clone();

        gxi! {
             Body [
                 div [
                    {
                        let weak_this1 = Rc::downgrade(&this);
                    },
                     button ( inner_html = "hello buddy", on_click = move |_| {
                         Self::update(&weak_this1, Msg::Panic);
                     }) ,
                     hello
                 ],
                 {
                    let weak_this2 = Rc::downgrade(&this);
                 },
                 h1 ( *inner_html = "23", on_click = move |_| {
                         Self::update(&weak_this2, Msg::Panic);
                 }) [
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
            Msg::Panic => Self::render(&this.upgrade().unwrap()),
        }
    }
}
