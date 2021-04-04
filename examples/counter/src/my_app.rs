use rust_gui::{*};

enum Msg { INC, DEC }
comp! {
    MyApp {
        count : u32 = 0;
        hello : String = String::from("Hello")
    }
    render {
        View [
            Image ( source = "cat.gif" ),
            View [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            if state.count == 5
                Text ( label=&state.count.to_string() )
        ]
    }
    update {
        match msg {
            Msg::INC => {
                state.count+=1;
            }
            _ => {
                if state.count > 0 {
                    state.count-=1;
                }
            }
        }
        ShouldRender::Yes
    }
}