use gxi::{gxi, render, set_state, update};

pub enum Msg {
    Modify(i32),
}

// TODO: Move gxi::Component macro from derive to functional macro, auto add node and state fields
#[derive(gxi::Component)]
pub struct Counter {
    node: gxi::ContainerNode,
    state: gxi::State<CounterState>,
}

#[derive(Default)]
pub struct CounterState {
    counter: i32,
}

// TODO: check if state.is_dirty()
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
