# Component

## Syntax

```rust
Name::constructor(..args) (..props) [
..children
]
```

### Name (required)

`$name` represents the name of the component.
It should be a valid [rust path](https://doc.rust-lang.org/reference/paths.html#paths-in-types).

**If**

`$name` has 0 path segments, no [`$constructor`](#constructor--default--newparent-) and starts
with a lowercase character then it is a `gxi::Element` constructed using the
`from_str(String, StrongNodeType)` associated function.

*e.g*
`gxi::NativeElement::from_str($name, parent).into_vnode_type()`

**else**

`$name::$constructor(parent).into_vnode_type()`

### Constructor ( default = new(parent) )

`$constructor` should be a lower-cased associated function of the component which returns `Self` and takes parent as the last argument

### args ( optional )

comma separated arguments passed to [`$constructor`](#constructor--default--newparent-)

### props

comma separated functions called on the component instance.

*e.g*

```rust
Comp::with_id("11") ( name = "aniket" )
```

should be transpiled to

```rust
Comp::with_id("11", parent)
.name("aniket")
.into_vnode_type()
```

**feature = "web"**

On enabling feature `web`, if `$prop.name` doesn't start with `on` or `set` then function `set(String,String)` is
invoked with args `set($prop.name, $prop.value)`

```rust
h1 ( title = "header one", set_name = "aniket" )
```

~

```rust
gxi::NativeElement::from_str("h1", parent)
.set("title", "header_one")
.set_name("aniket")
.into_vnode_type();
```

**right-hand values**

**if**

values on the right side of the assignment operator are static or independent of the environment then they shall be
called in the `init()` closure of the `init_member()` closure call.

*eg*

```rust
h1 ( title = "header one", set_name = "aniket" , on_click = |event| {}, value = state.name )
```

~

```rust
let (node, is_new) = init_member(node, InitType::Child, |parent| {
    gxi::NativeElement::from_str("h1", parent)
        .set("title", "header one")
        .set_name("aniket")
        .into_vnode_type()
}).unwrap();
```

**else if**

value expression is of type `closure` then it should be assigned once after
`initialisation` by checking the `is_new` flag.

```rust
let node_borrow = node.borrow_mut();
if is_new {
    node_borrow
        .on_click(|event| {

        });
}
```

**else**

other values which depend on the environment shall not be checked, updating them on each render call.

```rust
node_borrow
    .value(state.name);
```

### Children

0. Other Components

*recursive*

```rust
Comp [

]
```

> TODO

