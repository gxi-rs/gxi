use rust_gui::{*};

enum Msg { INC }
comp! {
    MyApp {
        count : u32 = 0;
        hello : String = String::from("Hello")
    }
    render {
        View [
            Image ( source = "cat.gif" ),
            View [
                Button ( label = "click", on_click = || Msg::INC )
            ],
            for x in 0..state.count
                if x % 2 == 0
                    Text ( label=&x.to_string() )
        ]
    }
    update {
        state.count+=1;
        ShouldRender::Yes
    }
}