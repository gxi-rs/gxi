# GXI

![Tests](https://github.com/gxi-rs/gxi/actions/workflows/tests.yml/badge.svg)
![Cross Platform Tests](https://github.com/gxi-rs/gxi/actions/workflows/cross-platform-tests.yml/badge.svg)

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
platform-independent components like `Text` and `View`. Therefore, making the code portable without losing deep control
of the native system.

## Platforms

+ [X] Desktop (GTK) Windows, Mac and Linux
+ [X] Web `wasm32-unknown-unknown`
+ [ ] Platform Independent (Web and GTK)
+ [ ] Android
+ [ ] Ios

## Examples

*Example For Desktop GTK App*

```rust
use crate::*;

enum Msg {
    INC,
    DEC,
}

gxi! {
    pub Counter {
        count : u32 = 0
    }
    render {
        View [
            View (orientation = Orientation::Vertical) [
                Button ( label = "Inc", on_click = || Msg::INC ),
                Button ( label = "Dec", on_click = || Msg::DEC )
            ],
            Text ( label = &state.count.to_string() )
        ]
    }
    update {
        let mut state = get_state_mut!(state);
        match msg {
            Msg::INC => state.count += 1,
            _ => {
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

//extend the abilities
impl Counter {
    pub fn count(&mut self, count: Option<u32>) {
        if let Some(count) = count {
            {
                let mut state = get_state_mut!(self.state);
                state.count = count;
            }
            self.mark_dirty();
        }
    }
}
```

*`Async` Example Using Web App*

```rust
use crate::*;
use serde_json::Value;

enum Msg {
    Fetch(bool),
}

gxi! {
    pub async CatFact {
        cat_fact : Option<String> = None
    }
    render {
        Init ( on_init = || Msg::Fetch(true) ) [
            Button ( class = "btn btn-dark" , on_click = || Msg::Fetch(false), inner_html = "Fetch Cat Memes" ),
            Div [
                if state.cat_fact.is_none() {
                    Div ( class = "spinner-border text-info" )
                } else {
                    H3 ( class = "text-light", inner_html = &state.cat_fact.as_ref().unwrap() )
                }
            ]
        ]
    }
    update {
        match msg {
            Msg::Fetch(force) => {
                if {
                    let mut state = get_state_mut!(state);
                    if state.cat_fact.is_some() {
                        state.cat_fact = None;
                        drop(state);
                        render();
                        true
                    } else {
                        false
                    }
                } || force
                {
                    let resp = reqwest::get("https://catfact.ninja/fact?max_length=140").await?;
                    let cat_fact:Value = serde_json::from_str(&resp.text().await?)?;
                    let mut state = get_state_mut!(state);
                    state.cat_fact = Some(cat_fact["fact"].to_string());
                    Ok(ShouldRender::Yes)
                } else {
                    Ok(ShouldRender::No)
                }
            }
        }
    }
}
```

More examples [here](examples)

## Contribution

Contributors can help by writing bindings.