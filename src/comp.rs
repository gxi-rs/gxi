mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::nodes::container::Container;
    use crate::nodes::node::Node;
    use crate::nodes::views::grid::Grid;
    use crate::nodes::widgets::button::Button;

    #[test]
    fn test() {
        let mut container: Rc<RefCell<Box<dyn Container>>> =
            Rc::new(RefCell::new(Box::new(Grid::default())));
        {
            container
                .borrow_mut()
                .get_child_mut()
                .get_or_insert_with(|| {
                    Node::Widget(Box::new(Button {
                        parent: container.clone(),
                    }))
                });
        }
    }
}
