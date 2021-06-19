# Changelog

## [v0.3.0-alpha.2] 14 Jun 2021

> Warn: Breaking changes

### Changes

- [X] execution block is no longer scoped

## [v0.3.0-alpha.1] 14 Jun 2021

> Warn: Breaking changes

### Changes

- [X] Complete unit tests for the compiler
- [X] Explicit Drop trait for Containers and Widgets
- [X] render function for widgets
- [X] Pass closure args
- [X] Fix #children call
- [X] Finish init_member()
- [X] Deref nodes
- [X] Change height and width from u32 to string in web binds

## [v0.2.0] 27 May 2021

> Warn: Breaking changes

### Changes

- re-wrote `gxi` macro
- `if` `for` `execution` blocks now require a comma separation
- reduced if code size
- performance increase
- drastically decreased code size by moving implementations from macro
to trait functions
- InitType no longer maintains a pure_index
- Reduced effective use of `Pure` component
- State Block is passed by `gxi!` macro instead of `macro_impl`
- new `pub` keyword for state and component
- setter methods using `pub` keyword before state fields
- Component Visual Scoping using Visual keywords like `pub`, `pub(crate)`
- `async` keyword moved from `update` block to component block
- `Window` `Body` are now `widget_components`
- the root of the tree can have only one component

## [v0.1.3] 5 May 2021

### Added
- `get_state!(state)` macro to get state
- Optimised state and reduced code size
- `async` update branch

### Removed
- `#[update(Name)]` macro removed

### Others
- `optimised` async routine code size
