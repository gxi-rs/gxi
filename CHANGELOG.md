# Changelog

## [v0.3.0-alpha.3] 14 Jun 2021

### Changes

- [x] reference nodes using `ref`
- [x] struct fields follow `viz`

## [v0.3.0-alpha.2] 14 Jun 2021

### Changes

- [x] execution block is no longer scoped
- [x] for loop will now take keys for non-linear data sources
- [x] constructor support
- [x] fixed web-sys export

## [v0.3.0-alpha.1] 14 Jun 2021

> Warn: Breaking changes

### Changes

- [x] Complete unit tests for the compiler
- [x] Explicit Drop trait for Containers and Widgets
- [x] render function for widgets
- [x] Pass closure args
- [x] Fix #children call
- [x] Finish init_member()
- [x] Deref nodes
- [x] Change height and width from u32 to string in web binds

## [v0.2.0] 27 May 2021

> Warn: Breaking changes

### Changes

- re-wrote `gxi` macro
- `if` `for` `execution` blocks now require a comma separation
- reduced if code size
- performance increase
- drastically decreased code size by moving implementations from macro to trait
  functions
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
- Optimized state and reduced code size
- `async` update branch

### Removed

- `#[update(Name)]` macro removed

### Others

- `optimized` async routine code size
