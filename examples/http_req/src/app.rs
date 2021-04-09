use rust_gui::*;

enum Msg {
    INC,
    DEC,
}

comp! {
    App {
        count : u32 = 0
    }
    render {
        View [
            Image ( source = "cat.gif" ),
            View [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            View [
                for x in 0..state.count
                    Text ( label = &x.to_string() )
            ],
            Button ( label = "Last" )
        ]
    }
    update {
        match msg {
            Msg::INC => {
                state.count+=1;
            }
            _ =>  {
                if state.count > 0 {
                    state.count-=1;
                } else {
                    return ShouldRender::No
                }
            }
        }
        ShouldRender::Yes
    }
}
