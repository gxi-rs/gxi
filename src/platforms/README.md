# Platforms

To achieve a common api for all platforms. Each platform needs to export 

1. a `run` function

```rust
    pub fn run<C: crate::VNode + crate::Renderable + 'static>() {}
```

2. `NativeWidget`
3. `NativeContainerWidget`
4. `Element` with `from_str(String, WeakNodeType) -> Self` 
