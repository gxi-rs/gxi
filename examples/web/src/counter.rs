use gxi::{gxi, render, set_state, update, comp};

pub enum Msg {
    Modify(i32),
}

#[comp]
pub struct Counter {
    counter: i32,
}

#[render(Counter)]
fn render(state: &gxi::State<CounterState>) {
    gxi! {
        div [
            h3 ( *inner_html = &state.counter.to_string()[..]),
            div [
                button ( inner_html = "+" , on_click = set_state!(Msg::Modify(1)) ),
                button ( inner_html = "-" , on_click = set_state!(Msg::Modify(-1)) ),
            ]
        ]
    }
}

#[update(Counter)]
fn update(msg: Msg, state: &mut Self::State) {
    match msg {
        Msg::Modify(by) => {
            state.counter += by;
        }
    }
}
