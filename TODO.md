# GXI

Cross-Platform Native Widget based Component System in Rust

## TO DO

### For release 0.3.0

- [ ] Complete unit tests for the compiler
- [ ] Complete the docs
- [X] Fix #children call
- [X] Finish init_member()
- [X] Deref nodes
- [X] Change height and width from u32 to string in web binds
- [ ] Generate Bindings for Desktop
- [ ] Organise imports
- [ ] Remove use statements from macro expansions
- [X] Explicit Drop trait for Containers and Widgets
- [X] render function for widgets
- [X] Pass closure args

### Completed ✓

- [x] Async Rendering
- [x] Comp Macro
- [x] `render!()` macro in update function
- [x] `Once` Component which executes update function on initialization
- [x] Call Render only when components are dirty
- [x] Derive macro for `update` function
- [x] Render child of a component from withing the component
- [X] Web Basic Support
- [X] mpsc channels for web

#### Planned

- [ ] gtk 4 support
- [ ] SSR support for all platforms
- [ ] Debug trait for nodes
