use crate::*;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::panic;

thread_local! {
    static TREE_ROOT: RefCell<Option<NodeRc>> = RefCell::new(None);
}

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
    TREE_ROOT.with(|root| {
        let mut tree_pointer = get_tree_pointer();
        // if tree pointer is 0, then it means it is null and not yet initialized
        let tree = if tree_pointer == 0 {
            let tree= Tree::new_node_tree();
            set_tree_pointer(Box::into_raw(Box::new(tree.clone())) as u32);
            tree
        } else {
            unsafe {
                let tree = tree_pointer as *const NodeRc;
                let tree = &*tree;
                tree.clone()
            }
        };
        //init child
        let tree_clone = Rc::downgrade(&tree);
        let app = tree.as_ref().borrow_mut().init_child(Box::new(|| App::new(tree_clone))).0;
        //render
        App::render(app);
        //replace root
        root.borrow_mut().replace(tree);
    });
}
