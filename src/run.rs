use std::cell::RefCell;
use std::rc::Rc;

use gtk::WidgetExt;

use crate::{AsyncNode, Node, Window, Fake};

pub fn run<App: Node>() {
    gtk::init().unwrap();
    let window: AsyncNode = Window::new(Rc::new(RefCell::new(Box::new(Fake{}))), None);
    //render
    {
        let widget = Some(window.as_ref().borrow().get_widget_as_container());
        App::render(App::new(window.clone(), widget));
    }
    //show window
    {
        let mut window_borrow = window.as_ref().borrow_mut();
        let window = window_borrow.as_any_mut().downcast_mut::<Window>().unwrap();
        window.widget.show_all();
    }
    //start main loop
    gtk::main();
}
