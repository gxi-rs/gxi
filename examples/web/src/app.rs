use rust_gui::*;

enum Msg {
    INC,
    DEC,
}

comp! {
    App {
        count: u32 = 0
    }
    render {
        Div [
            Body ( style = r#"background-color : #121212;"# ),
            Head [
                Title ( inner_html = "Hello World" ),
                Link ( rel = "stylesheet", href = "https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" ),
                Meta ( name = "viewport", content = "width=device-width, initial-scale=1" )
            ],
            A ( href = "https://webbuddy360.com" ) [
                H1 ( label = "hello world" ),
            ],
            Div [
                Button ( label = "Inc", on_click = || Msg::INC , class="btn btn-dark"),
                Button ( label = "Dec", on_click = || Msg::DEC , class="btn btn-light")
            ],
            H2 ( label = &state.count.to_string() , class = "text-info")
        ]
    }
}

#[update(App)]
async fn update(args: UpdateArgs) -> AsyncResult<ShouldRender> {
    let UpdateArgs { msg, state, .. } = args;
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
