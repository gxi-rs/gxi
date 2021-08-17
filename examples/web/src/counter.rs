use crate::*;

enum Msg {
    INC,
    DEC,
}

gxi! {
    pub Counter {
        count: u32 = 0
    }
    render {
        Div [
            Div [
                Button ( inner_html = "Inc", on_click = |_| Msg::INC , class="btn btn-dark"),
                Button ( inner_html = "Dec", on_click = |_| Msg::DEC , class="btn btn-light")
            ],
            H2 ( inner_html = &state.count.to_string() , class = "text-info")
        ]
    }
    update {
        let mut state = get_state_mut!(state);
        match msg {
            Msg::INC => {
                state.count += 1;
            }
            _ => {
                if state.count > 0 {
                    state.count -= 1;
                } else {
                    return Ok(ShouldRender::No);
                }
            }
        }
        Ok(ShouldRender::Yes)
    }
}
