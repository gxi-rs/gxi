mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let mut container: Rc<RefCell<Box<dyn Container>>> =
            Rc::new(RefCell::new(Box::new(views::Grid::default())));
        {
            container
                .borrow_mut()
                .get_child_mut()
                .get_or_insert_with(|| {
                    Node::Widget(Box::new(widgets::Button {
                        parent: container.clone(),
                    }))
                });
        }
    }
}
