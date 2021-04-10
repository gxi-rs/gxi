# RustGUI

Cross-Platform Native Widget based Component System in Rust

🎉🥂🥳 `Async` Support is here

This project targets cross platform GUI app development with react like function component and state management.
Built in procedural macros to help devs write minimum code.

*Example*
```rust
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
}

#[update(Counter)]
async fn update<F: Fn() + 'static>(state: AsyncState, msg: Msg, _render: F) -> AsyncResult<ShouldRender> {
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
```

More examples [here](examples)

# Status
This project is stable but in very early stage. Complete support for Linux will be released first, then web, and then the rest.

Operating systems:
+ [x] Linux (GTK)
+ [ ] Mac
+ [ ] Windows
+ [ ] Web
+ [ ] Android
+ [ ] Ios

# Contribution
Contributors can help by writing bindings.