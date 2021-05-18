use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use crate::*;
/*
thread_local! {
    static TREE_ROOT: RefCell<Option<NodeRc>> = RefCell::new(None);
}
*/
/*
window.set_tree_pointer = function (pointer) {
          window.tree_pointer = pointer;
      }
      window.get_tree_pointer = function () {
          return window.tree_pointer | 0;
      }
*/

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn set_tree_pointer(pointer_to_tree: u32);
    #[wasm_bindgen()]
    fn get_tree_pointer() -> u32;
}

pub fn run<App: Node + 'static>() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    //    TREE_ROOT.with(|root| {
    {
        let tree_pointer = get_tree_pointer();
        // if tree pointer is 0, then it means it is null and not ye initialized
        let app = if tree_pointer == 0 {
            let fake_parent: NodeRc = Rc::new(RefCell::new(Box::new(Fake)));
            let body = Body::new(Rc::downgrade(&fake_parent));
            set_tree_pointer(Box::into_raw(Box::new(body.clone())) as u32);
            //init child
            let app = {
                let body_clone = Rc::downgrade(&body);
                body.borrow_mut()
                    .init_child(Box::new(|| App::new(body_clone))).0
            };
            app
        } else {
            let body = tree_pointer as *mut NodeRc;
            let body = unsafe { &*body };
            let body_child = body.borrow();
            let body_child = body_child.get_child();
            let body_child = body_child.as_ref().unwrap();
            body_child.clone()
        };
        App::render(app);
    }
    //});
}
