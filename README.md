# RustGUI

CrossPlatform Native Widget based Component System in rust

This project targets cross platform GUI app development with react like function component and state management.
Built in procedural macros to help devs write minimum code.

*Examples*
```rust
c!(
    View {
        {
            if state.count % 2 == 0 {
                c! ( 1 Button { set_label="Eve"; } );
            } else {
                c! ( 2 Button {set_label="Odd"; });
            }
        }
    },
    View [
        Button { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; },
    ]
);
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