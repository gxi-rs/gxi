use std::ops::DerefMut;

use std::sync::{Arc, Mutex};

use crate::nodes::node::{AsyncNode, Node};
use crate::nodes::views::grid::Grid;
use crate::nodes::widgets::button::Button;

mod nodes;

fn main() {
    let container: AsyncNode = Arc::new(Mutex::new(Node::Container(Box::new(Grid::default()))));

    if let Node::Container(s) = container.clone().lock().unwrap().deref_mut() {
        s.get_child_mut().get_or_insert_with(|| {
            Arc::new(Mutex::new(Node::Widget(Box::new(Button {
                parent: Some(container.clone()),
                ..Default::default()
            }))))
        });
        println!("container");
    }
}
