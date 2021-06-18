mod common;
pub use common::*;
pub use gxi::*;

gxi! {
    pub App {
        limit : u32 = 0
    }
    render {
        crate::common::Comp::from_class_and_id("h1","12") [
            { println!("render"); },
            if state.limit == 0 {
                { println!("true"); },
                Comp::from_class("h1") ( id = "asd" ) [
                    Comp::from_class("1"),
                    Comp ( id = "1", class = "12" ),
                    for x in 0..2 {
                        { println!("{}",x); },
                        Comp [
                            { println!("{}", x); }
                        ]
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
        let node = check_child_type::<Comp>(app, "Comp");
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
                    no_sibling(node);
                }
                no_sibling(node);
            }
            let node = check_sibling_type::<Comp>(node, "Comp");
            no_sibling(node);
        }
        no_sibling(node);
    }
}
