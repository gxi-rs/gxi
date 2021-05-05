use gxi::*;

enum Msg {
    INC,
    DEC,
}

gxi! {
    Counter {
        count : u32 = 0
    }
    render {
        View [
            View (orientation = Orientation::Vertical) [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            Text ( label = &state.count.to_string() ),
            View (orientation = Orientation::Vertical) [
                for i in 0..2
                    Text ( label = &i.to_string() )
            ]
        ]
    }
    update {
        let mut state = get_state!(state);
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
                let mut state = get_state!(self.state);
                state.count = count;
            }
            self.mark_dirty();
        }
    }
}
