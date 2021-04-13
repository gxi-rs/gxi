use rust_gui::*;

comp! {
    App {}
    render {
        Div [
            H1 ( label = "Hello" ),
            H1 ( label = "World" )
        ]
    }
}
