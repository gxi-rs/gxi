use crate::*;
use rust_gui_interface::wasm_bindgen::JsCast;

create_widget!(Button);

impl_web_container!(Button,"button");

impl Button {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
    pub fn on_click<F: Fn() + 'static>(&self, f: F) {
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            f();
        }) as Box<dyn Fn(_)>);
        self.widget.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}