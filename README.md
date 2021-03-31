# RustGUI

Cross-Platform Native Widget based Component System in Rust

This project targets cross platform GUI app development with react like function component and state management.
Built in procedural macros to help devs write minimum code.

*Example*
```rust
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