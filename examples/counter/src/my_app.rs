use rust_gui::{*};

enum Msg { INC }
comp! {
    MyApp {
        count : u32 = 0;
        hello : String = String::from("Hello")
    }
    render {
        View [
            View [
                Button ( set_label = "click", connect_clicked = || Msg::INC )
            ],
            for x in 0..state.count
                if x % 2 == 0
                    Button ( set_label=&x.to_string() )
        ]
    }
    update {
        state.count+=1;
        ShouldRender::Yes
    }
}