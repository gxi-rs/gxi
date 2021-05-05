# GXI

*Cross-Platform Native Widget based Component System in Rust*

🎉🥂🥳 `Async` Support is here

This project implements a component system of GUI widgets and nodes. Using
rust [proc-macros](https://doc.rust-lang.org/reference/procedural-macros.html) compiles the component tree to optimized
logical n-binary tree flow which `prevents` the use of any [virtual dom](https://reactjs.org/docs/faq-internals.html)
or [diffing algorithms](https://reactjs.org/docs/reconciliation.html). Making the component system `zero cost`. Hence
the components are
`highly optimized`, `performant`, and `customized` to meet the needs of each project while maintaining the standard
features of frameworks like `React`. Built-in `async support` allows for quick and performant abstractions to rust
futures.

Since the framework is a compiler, therefore, it allows mixing of platform dependent and independent components, i.e the
framework provides components like `div`, `h1` (platform dependent) and [React Native](https://reactnative.dev/) like
platform-independent components like `Text` and `View`. Therefore making the code portable without losing deep control
of the native system.

## Platforms

+ [X] Desktop (GTK) Windows, Mac and Linux
+ [X] Web `wasm32-unknown-unknown`
+ [X] Platform Independent (Web and GTK)
+ [ ] Android
+ [ ] Ios

## Examples

*Example For Desktop GTK App*

```rust
use gxi::*;

enum Msg {
    INC,
    DEC,
}

gxi! {
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
    update {
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
}
```

*Example For Web App*

```rust
use gxi::*;

enum Msg {
    INC,
    DEC,
}

gxi! {
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
async fn update<F: Fn() + 'static>(state: State, msg: Msg, _render: F) -> AsyncResult<ShouldRender> {
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

## Contribution

Contributors can help by writing bindings.