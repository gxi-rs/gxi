pub use common::*;
pub use gxi::*;

mod common;
gxi! {
    pub App {
        limit : u32 = 0,
        names: Vec<Rc<String>> = vec![Rc::new(String::from("A")), Rc::new(String::from("B"))],
        comp_ref : Option<WeakNodeType>
    }
    render {
        crate::common::Comp::from_class_and_id("h1","12") [
            { println!("render"); },
            if state.limit == 0 {
                { println!("true"); },
                Comp::from_class("h1") ( id = "asd" ) [
                    Comp::from_class("1"),
                    Comp ( ref = state.comp_ref, id = "1", class = "12" ),
                    for x in 0..2 {
                        { println!("{}",x); },
                        Comp [
                            { println!("{}", x); }
                        ]
                    },
                    for x in &state.names where x:Rc<String> {
                        Comp
                    }
                ],
                { println!("true"); }
            } else {
                Foo
            },
            Comp [

            ]
        ],
        { println!("render complete"); },
    }
}

#[test]
fn traverse() {
    let root = Root::new_root();
    let (app, ..) = init_member(root.clone(), InitType::Child, |this| App::new(this), true);
    App::render(app.clone());
    //start traversing app
    {
        let node = check_child_type::<Comp>(app.clone(), "Comp");
        {
            let node = check_substs_child_type::<Pure>(node.clone(), "Pure");
            {
                let node = check_substs_child_type::<Comp>(node.clone(), "Comp");
                {
                    let node = check_substs_child_type::<Comp>(node.clone(), "Comp");
                    let node = check_sibling_type::<Comp>(node, "Comp");
                    // for loop
                    let node = check_sibling_type::<Pure>(node, "Pure");
                    {
                        let node = check_substs_child_type::<Pure>(node.clone(), "Pure");
                        // for loop runs twice
                        let node = check_sibling_type::<Comp>(node, "Comp");
                        let node = check_sibling_type::<Comp>(node, "Comp");

                        no_sibling(node);
                    }
                    // for loop 2
                    let node =
                        check_sibling_type::<ForWrapper<Rc<String>>>(node.clone(), "ForWrapper");
                    {
                        let node_borrow = node.as_ref().borrow();
                        let node = node_borrow
                            .as_node()
                            .as_any()
                            .downcast_ref::<ForWrapper<Rc<String>>>()
                            .unwrap();

                        for (.., (child, exists)) in &node.children {
                            let node = check_child_type::<Comp>(child.clone(), "Comp");
                            no_sibling(node);
                            assert_eq!(exists, &false);
                        }
                    }
                    no_sibling(node);
                }
                no_sibling(node);
            }
            let node = check_sibling_type::<Comp>(node, "Comp");
            no_sibling(node);
        }
        no_sibling(node);
    }
    // ref test
    {
        let app = app.as_ref().borrow();
        let app = app.as_node().as_any().downcast_ref::<App>().unwrap();
        let state = app.state.as_ref().borrow();
        let comp_ref = state.comp_ref.as_ref().unwrap();
        unwrap_weak_node!(comp_ref as Comp);
        let comp_state = comp_ref.state.as_ref().borrow();
        assert_eq!(comp_state.class, "12");
        assert_eq!(comp_state.id, "1");
    }
}
