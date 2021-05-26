use gxi::*;
use crate::macros::gxi_macro::comp::Comp;

gxi! {
    App {
        limit : u32 = 2
    }
    render {
        /* Top comment */
        Comp [
            { println!("render"); },
            if state.limit == 2 {
                { println!("true"); },
                Comp ( class = "h1".to_string(), id = "asd".to_string() ) [
                    Comp,
                    Comp
                ],
                { println!("true"); }
            } else {
                Comp [

                ]
            }
        ],
        { println!("render complete"); },
    }
}

#[test]
fn traverse_conditional() {
    let root = Root::new_root();
    let app = {
        let root_weak = Rc::downgrade(&root);
        root.borrow_mut().init_child(Box::new(|| App::new(root_weak))).0
    };
    //render
    App::render(app.clone());
    //start traversing app
    {
        let node = check_child_type::<Comp>(app, "Comp");
        check_sibling_type::<Comp>(node, "Comp");
    }
}

fn check_child_type<T: 'static + Node>(node: NodeRc, name: &str) -> NodeRc {
    let node_borrow = node.as_ref().borrow();
    let child = node_borrow.get_child().as_ref().unwrap();
    child.as_ref().borrow().as_any().downcast_ref::<T>().expect(&format!("expected '{}' here", name));
    child.clone()
}

fn check_sibling_type<T: 'static + Node>(node: NodeRc, name: &str) -> NodeRc {
    let node_borrow = node.as_ref().borrow();
    let child = node_borrow.get_sibling().as_ref().unwrap();
    child.as_ref().borrow().as_any().downcast_ref::<T>().expect(&format!("expected '{}' here", name));
    child.clone()
}