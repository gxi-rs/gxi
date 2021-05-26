use crate::*;

enum Msg {
    INC,
    DEC,
}

gxi! {
    pub Counter {
        count : u32 = 0
    }
    render {
        View [
            View (orientation = Orientation::Vertical) [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            Text ( label = &state.count.to_string() )
        ]
    }
    update {
        let mut state = get_state_mut!(state);
        match msg {
            Msg::INC => state.count += 1,
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

impl Counter {
    pub fn count(&mut self, count: Option<u32>) {
        if let Some(count) = count {
            {
                let mut state = get_state_mut!(self.state);
                state.count = count;
            }
            self.mark_dirty();
        }
    }
}
