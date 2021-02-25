use crate::components::Component;
use crate::MyApp;

pub fn run<T: Component + Default>() {
    /*let mut app = T::default();
    app.render();*/
    let mut  app = MyApp::default();
    app.render();
    app.render();
    app.state.counter = 2;
    app.render();
    app.state.counter = 0;
    app.render();
    app.render();
    app.render();
    app.render();
}