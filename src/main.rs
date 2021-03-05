use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::container::Container;
use crate::nodes::node::Node;
use crate::nodes::views::grid::Grid;
use crate::nodes::widgets::button::Button;
use std::ops::DerefMut;

mod nodes;

fn main() {
    let container = Rc::new(RefCell::new(Node::Container(Box::new(Grid::default()))));

    let clone = container.clone();
    match clone.borrow_mut().deref_mut() {
        Node::Container(s) => {

        }
        _ => {}
    }
    if let Node::Container(s) = container.clone().get_mut() {
        s.get_child_mut().get_or_insert_with(|| {
            Rc::new(RefCell::new(Node::Widget(Box::new(Button {
                parent: Some(container.clone()),
                ..Default::default()
            }))))
        });
    }
}

