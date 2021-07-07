pub fn run<C: crate::VNode + crate::Renderable + 'static>() {
    let mut node = C::new();
    node.render();
}
