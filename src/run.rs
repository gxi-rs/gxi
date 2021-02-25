use crate::components::Component;

pub fn run<T: Component + Default>() {
    let mut app = T::default();
    app.render(true);
}