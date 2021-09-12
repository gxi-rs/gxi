use gxi::Body;
use gxi::{gxi, init_member, InitType, StrongNodeType, VNode};

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

enum Msg {
    Panic,
}

impl gxi::Renderable for App {
    fn render(__node: &StrongNodeType) {
        gxi! {
            Body [
                div () [
                    button ( inner_html = "hello buddy" ),
                    hello
                ],
                h1 ( inner_html = "23", on_click = |_| panic!("hello") ) [
                    h1 ( inner_html = "12" ),
                    h1 ( inner_html = "12" )
                ]
            ]
       }
    }
}
