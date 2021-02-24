use std::collections::HashMap;

use crate::component::Component;
use crate::MyApp;

pub fn run<T: 'static + Component + Default>() {
    let mut tree: HashMap<i32, Box<dyn Component>> = HashMap::new();
    MyApp::default().render(&mut tree, true);
}