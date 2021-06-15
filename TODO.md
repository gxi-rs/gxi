# GXI

Cross-Platform Native Widget based Component System in Rust

### TO DO

## For release 0.3.0

- [ ] Complete the docs
- [ ] Generate Bindings for Desktop
- [ ] Organise imports
- [ ] Remove use statements from macro expansions
- [ ] Add support for constructor args
- [ ] Add support for functions that don't take no params
- [ ] Add if-else block in params block  

## Others

- [ ] gtk 4 support
- [ ] SSR support for all platforms
- [ ] Debug trait for nodes

### Completed ✓

- [X] Change height and width from u32 to string in web binds
- [X] Complete unit tests for the compiler
- [X] Fix #children call
- [X] Finish init_member()
- [X] Deref nodes
- [X] Explicit Drop trait for Containers and Widgets
- [x] Async Rendering
- [x] Comp Macro
- [x] `render!()` macro in update function
- [x] `Once` Component which executes update function on initialization
- [x] Render call only when custom components are flagged dirty
- [x] Derive macro for `update` function
- [x] Render child of a component from withing the component
- [X] Web Basic Support
- [X] mpsc channels for web
