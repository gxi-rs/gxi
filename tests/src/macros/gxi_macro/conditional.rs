use crate::macros::gxi_macro::{Comp, Foo};
use gxi::*;

gxi! {
    pub App {
        limit : u32 = 0
    }
    render {
        /* Top comment */
        crate::macros::gxi_macro::comp::Comp [
            { println!("render"); },
            if state.limit == 0 {
                { println!("true"); },
                Comp ( class = "h1".to_string(), id = "asd".to_string() ) [
                    Comp,
                    Comp,
                    for x in 0..2 {
                        { println!("{}",x); },
                        Comp [
                            { println!("{}", x); }
                        ]
                    }
                ],
                { println!("true"); }
            } else {
                Foo [
                ]
            },
            Comp [

            ]
        ],
        { println!("render complete"); },
    }
}

#[cfg(test)]
mod tests {
    use crate::macros::gxi_macro::{App, Comp};
    use gxi::*;

    #[test]
    fn traverse_conditional() {
        let root = Root::new_root();
        let (app, ..) = init_member(root.clone(), InitType::Child, |this| App::new(this));
        App::render(app.clone());
        //start traversing app
        {
            let node = check_child_type::<Comp>(app, "Comp");
            {
                let node = check_child_type::<Pure>(node.clone(), "Pure");
                {
                    let node = check_child_type::<Comp>(node.clone(), "Comp");
                    {
                        let node = check_child_type::<Comp>(node.clone(), "Comp");
                        let node = check_sibling_type::<Comp>(node, "Comp");
                        // for loop
                        let node = check_sibling_type::<Pure>(node, "Pure");
                        {
                            let node = check_child_type::<Pure>(node.clone(), "Pure");
                            // for loop runs twice
                            let node = check_sibling_type::<Comp>(node, "Comp");
                            let node = check_sibling_type::<Comp>(node, "Comp");
                            
                            no_siblibng(node);
                        }
                        no_siblibng(node);
                    }
                    no_siblibng(node);
                }
                let node = check_sibling_type::<Comp>(node, "Comp");
                no_siblibng(node);
            }
            no_siblibng(node);
        }
    }

    fn no_siblibng(node: StrongNodeType) {
        let node_borrow = node.as_ref().borrow();
        if node_borrow.as_node().get_sibling().is_some() {
            panic!("no sibling was expected");
        }
    }

    fn check_child_type<T: 'static + Node>(node: StrongNodeType, name: &str) -> StrongNodeType {
        let node_borrow = node.as_ref().borrow();
        let child = node_borrow
            .as_container()
            .unwrap()
            .get_child()
            .as_ref()
            .unwrap()
            .clone();
        let child_borrow = child.as_ref().borrow();
        child_borrow
            .as_node()
            .as_any()
            .downcast_ref::<T>()
            .expect(&format!("expected '{}' here", name));
        drop(child_borrow);
        child
    }

    fn check_sibling_type<T: 'static + Node>(node: StrongNodeType, name: &str) -> StrongNodeType {
        let node_borrow = node.as_ref().borrow();
        let sibling = node_borrow
            .as_node()
            .get_sibling()
            .as_ref()
            .unwrap()
            .clone();
        let sibling_borrow = sibling.as_ref().borrow();
        sibling_borrow
            .as_node()
            .as_any()
            .downcast_ref::<T>()
            .expect(&format!("expected '{}' here", name));
        drop(sibling_borrow);
        sibling
    }
}
