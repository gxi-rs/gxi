# Gxi-Transpiler
*transpiler for gxi-rs component system*

## Syntax

```rust
Name::constructor(..args) (..props) [
    ..children
]
```

## Component

### Name (required)
> $name` represents name of the component

If $name starts with

1. `lower-case` = `gxi::NativeElement::from_str($name, parent).into_vnode_type()`
2. `upper-case` = `$name::constructor(parent).into_vnode_type()`


### Constructor ( default = $name::new(parent) )

`$constructor` may be an associated function of the component which returns `Self` and takes parent as the last argument

### args ( optional )

arguments passed to the `constructor` function.

### props

comma separated functions called on the component instance.

*e.g*

```rust
Comp::with_id("12") ( name = "aniket" )
```

is equal to

```rust
Comp::with_id("12", parent)
    .name("aniket")
    .into_vnode_type()
```

**feature = "web"**

On enabling feature `web`, if $prop.name doesn't start with `on` or `set` then
function `set` is invoked with args `.set($prop.name, $prop.value)`

```rust
h1 ( title = "header one", set_name = "aniket" )
```

which is equal to

```rust
gxi::NativeElement::from_str($name, parent)
  .set("title", "header_one")
  .set_name("aniket")
  .into_vnode_type()
```

> TODO: literal values

### Children

1. Other Components

*recursive*

```rust
Comp [

]
```

> TODO

