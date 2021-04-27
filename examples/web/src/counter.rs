use crate::*;

enum Msg {
    INC,
    DEC,
}

comp! {
    Counter {
        count: u32 = 0
    }
    render {
        Div [
            Button ( inner_html = "Inc", on_click = || Msg::INC , class="btn btn-dark"),
            Button ( inner_html = "Dec", on_click = || Msg::DEC , class="btn btn-light")
        ],
        H2 ( inner_html = &state.count.to_string() , class = "text-info")
    }
}

#[update(Counter)]
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
