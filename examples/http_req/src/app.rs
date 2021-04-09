use rust_gui::*;

enum Msg {
    Fetch
}

comp! {
    App {
        corona_count : Option<u32> = None
    }
    render {
        View [
            if state.corona_count.is_none()
                Spinner
            else
                Text ( label = &format!("Today's Corona Count is {}", state.corona_count.unwrap()))
        ]
    }
    update {
        match msg {
            Msg::Fetch => {
                state.corona_count = Some(20);
            }
        }
        ShouldRender::Yes
    }
}
