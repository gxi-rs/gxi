# GXI

![Tests](https://github.com/gxi-rs/gxi/actions/workflows/tests.yml/badge.svg)

*Zero-Cost Cross-Platform Native Widget based Component System in Rust*

Using [proc-macros](https://doc.rust-lang.org/reference/procedural-macros.html),
the [gxi transpiler](gxi-transpiler/README.md) transpiles a component tree into an n-binary tree order, without using
any [virtual dom](https://reactjs.org/docs/faq-internals.html)
or a [diffing algorithm](https://reactjs.org/docs/reconciliation.html). The component system is platform-agnostic, which
allow the system to produce platform dependent and independent components, merging them together for code reuse and
maintainability.

At compile time the gxi-transpiler compares the tree to predefined optimised tree structures for widgets, components and
top level nodes, generating highly performant render calls and async/sync state management.

## Platforms

+ [ ] Desktop (GTK) Windows, Mac and Linux
+ [ ] Web `wasm32-unknown-unknown`
+ [ ] Platform Independent (Web and GTK)
+ [ ] Android
+ [ ] Ios

## Examples

```rust
use gxi::*;

#[derive(Component)]
pub struct App {
  node: ContainerNode,
}

impl Renderable for App {
  fn render(node: &StrongNodeType) {
    gxi! {
      Body { // top level node
        h1 { // native widget
          "title"
        },
        Counter::with_data(23) ( min = 24, max = 124 ) // user defined component
      }
    };
  }
}
```

Full src [here](examples)

## Code of conduct

Code of conduct can be found at **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)**

## Contributing

Make sure to read **[Contribution Guidelines](CONTRIBUTING.md)** before contributing.

## License & Copyright

Copyright (C) 2020 Aniket Prajapati

Licensed under the **[MIT LICENSE](LICENSE)**

## Contributors

+ [Aniket Prajapati](https://aniketprajapati.me)
  @[prajapati.ani306@gmail.com](mailto:contact@aniketprajapati.me
