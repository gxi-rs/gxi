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
macro_rules! cont {
    ($e:ident ()) => {
        let container = {
            let mut container_borrow = container.as_ref().borrow_mut();
            let container = Rc::clone(&container);
            container_borrow
                .init_child(Box::new(move || $e::new(container.clone())), true)
                .0
        };
        println!(":asd");
    }
}
