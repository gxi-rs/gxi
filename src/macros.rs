#[macro_export]
macro_rules! con {
    ($e:ident ($container:expr)) => {{
        let container = Rc::clone(&$container);
        let mut container_borrow = container.as_ref().borrow_mut();
        let container = Rc::clone(&container);
        container_borrow
            .init_child(Box::new(move || $e::new(container.clone())), true)
            .0
    };};
}

#[macro_export]
macro_rules! nod {
    ($Node:ident ($prev_node:expr, $type:ident ,$state:expr) {} {$($f:ident = $v:ident)?}) => {{
        let (node, _is_new) = {
            let mut node_borrow = $prev_node.as_ref().borrow_mut();
            let container = Rc::clone(&$prev_node);
            node_borrow.$type(Box::new(move || $Node::new(container.clone())), true)
        };
        {
            let mut node_borrow = node.as_ref().borrow_mut();
            let node = node_borrow.as_any_mut().downcast_mut::<$Node>().unwrap();
            let state = $state.as_ref().borrow();
            $(
                node.widget.$f(state.$v.to_string().as_str());
            )*
        }
        node
    }};
}
