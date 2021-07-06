pub fn run<C: gxi::VNode + gxi::Renderable + 'static>() {
    let mut node = C::new();
    node.render();
}
