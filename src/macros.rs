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
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ButtonExt, ContainerExt, WidgetExt, WindowType};

use crate::nodes::containers::pure::Pure;
use crate::nodes::containers::view::View;
use crate::nodes::containers::window::Window;
use crate::nodes::node::{AsyncNode, Node};
use crate::nodes::widgets::button::Button;

#[macro_export]
macro_rules! cont {
    ($e:ident ($container:expr)) => {
        let cont = {
            let mut container_borrow = $container.as_ref().borrow_mut();
            let container = Rc::clone(&$container);
            container_borrow
                .init_child(Box::new(move || $e::new($container.clone())), true)
                .0
        };
    };
}
