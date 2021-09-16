
use gxi::{gxi, StrongNodeType};
use gxi::{update, Body, WeakNodeType};

#[derive(gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

enum Msg {
    ReRender,
    CustomPanic(&'static str),
}

impl gxi::Renderable for App {
    fn render(this: &StrongNodeType) {
        //TODO: add unwrap line numbers
        gxi! {
             Body [
                 div [
                     button ( inner_html = "hello buddy", on_click = update!(Msg::ReRender) ),
                     hello
                 ],
                 h1 ( inner_html = "23", on_click = update!(Msg::CustomPanic("hello")) ) [
                     h1 ( inner_html = "12" ),
                     h1 ( inner_html = "12" )
                 ],
                 crate::Counter,
                 {
                    crate::Counter::render(&__node.clone());
                 } 
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
