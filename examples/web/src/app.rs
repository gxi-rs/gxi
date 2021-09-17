use gxi::{gxi, render, set_state, update, Body};

#[derive(gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

#[derive(Default)]
pub struct AppState {

}

pub enum Msg {
    ReRender,
    CustomPanic(&'static str),
}

#[render(App)]
fn render() {
    gxi! {
      Body [
         div [
            button ( inner_html = "hello buddy", on_click = set_state!(Msg::ReRender) ),
            hello
         ],
         h1 ( inner_html = "23", on_click = set_state!(Msg::CustomPanic("hello")) ) [
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

#[update(App)]
fn update(msg: Self::Msg) {
    match msg {
        Msg::ReRender => (),
        Msg::CustomPanic(msg) => {
            panic!("{}", msg)
        }
    }
}
