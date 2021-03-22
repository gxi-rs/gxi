/*#[macro_export]
macro_rules! comp  {
    {$e:ident { $( $n:ident )? }} => {
        fn render(top_container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
            let container = Rc::clone(&top_container);
            let container = {
                let mut container_borrow = container.as_ref().borrow_mut();
                let container = Rc::clone(&container);
                container_borrow.init_child(Box::new(move || $e::new(container.clone())), true).0

            };
        }
    };
}
*/

#[macro_export]
macro_rules! con {
    ($e:ident ($container:expr)) => {
        {
            let container = Rc::clone(&$container);
            let mut container_borrow = container.as_ref().borrow_mut();
            let container = Rc::clone(&container);
            container_borrow
                .init_child(Box::new(move || $e::new(container.clone())), true)
                .0
        };
    };
}


#[macro_export]
macro_rules! nod {
    ($e:ident ($prev_node:expr, $type:ident )) => {
        {
            let mut node_borrow = $prev_node.as_ref().borrow_mut();
            let container = Rc::clone(&$prev_node);
            node_borrow.$type(Box::new(move || $e::new(container.clone())), true).0
        };
    };
}