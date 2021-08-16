use gxi::Body;
use gxi::{gxi, init_member, InitType, StrongNodeType, VNode};

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

impl gxi::Renderable for App {
    fn render(node: &StrongNodeType) {
        gxi! {
            Body
        };

        {
            let cont = node.clone();

            gxi! {
                h1
            };
            {
                gxi! {
                    h1
                }
            }
            let _ = init_member(&node, InitType::Sibling(&cont), |parent| {
                gxi::Element::from_str("h1", parent).into_vnode_type()
            })
            .unwrap();
        }
    }
}
