use std::collections::HashMap;

/*struct Component<T: Default> {
    //This can be anything that implements default
    root: T,
    //children
    nodes: HashMap<i32, Box<Component<T>>>,
}*/

pub type Nodes = HashMap<i32, Box<dyn Component>>;

pub trait Component {
    fn get_nodes(&self) -> Option<&Nodes>;
    fn get_mut_nodes(&mut self) -> Option<&mut Nodes>;
    fn render(&self, tree: &mut Nodes, first: bool);
}