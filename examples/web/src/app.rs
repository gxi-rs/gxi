use rust_gui::*;

enum Msg {}
comp! {
    App {}
    render {
        Div [
            H1 ( label = "Hello" ),
            H1 ( label = "World" )
        ]
    }
}

#[update(App)]
async fn update<F: Fn() + 'static>(state: AsyncState, msg: Msg, _render: F) -> AsyncResult<ShouldRender> {
    Ok(ShouldRender::Yes)
}
