# GXI

Cross-Platform Native Widget based Component System in Rust

### TO DO

## For release 0.3.0

- [ ] Complete unit tests for the compiler
- [ ] Complete the docs
- [ ] Fix #children call
- [ ] Finish init_member()
- [ ] Deref nodes
- [ ] Change height and width from u32 to string in web binds
- [ ] Generate Bindings for Desktop
- [ ] Organise imports
- [ ] Remove use statements from macro expansions
- [ ] Explicit Drop trait for Containers and Widgets

## Others

- [ ] gtk 4 support
- [ ] SSR support for all platforms
- [ ] Debug trait for nodes

### Completed ✓

- [x] Async Rendering
- [x] Comp Macro
- [x] `render!()` macro in update function
- [x] `Once` Component which executes update function on initialization
- [x] Render call only when custom components are flagged dirty
- [x] Derive macro for `update` function
- [x] Render child of a component from withing the component
- [X] Web Basic Support
- [X] mpsc channels for web
