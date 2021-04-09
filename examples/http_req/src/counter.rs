use rust_gui::*;

enum Msg {
    INC,
    DEC,
}

comp! {
    Counter {
        count : u32 = 0
    }
    render {
        View [
            View [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            Text ( label = &state.count.to_string() )
        ]
    }
    update {
        match msg {
            Msg::INC => {
                let mut state = state.lock().unwrap();
                state.count+=1;
            }
            _ =>  {
                let mut state = state.lock().unwrap();
                if state.count > 0 {
                    state.count-=1 ;
                } else {
                    return Ok(ShouldRender::No)
                }
            }
        }
        Ok(ShouldRender::Yes)
    }
}
