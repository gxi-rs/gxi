use rust_gui::*;

enum Msg {
    INC,
    DEC
}

comp! {
    App {
        count: u32 = 0
    }
    render {
        Div [
            El [

            ],
            Div [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            H1 ( label = &state.count.to_string() )
        ]
    }
}

#[update(App)]
async fn update<F: Fn() + 'static>(
    state: AsyncState, msg: Msg, _render: F,
) -> AsyncResult<ShouldRender> {
    match msg {
        Msg::INC => {
            let mut state = state.lock().unwrap();
            state.count += 1;
        }
        _ => {
            let mut state = state.lock().unwrap();
            if state.count > 0 {
                state.count -= 1;
            } else {
                return Ok(ShouldRender::No);
            }
        }
    }
    Ok(ShouldRender::Yes)
}