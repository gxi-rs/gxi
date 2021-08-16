use gxi::Body;
use gxi::{gxi, init_member, InitType, StrongNodeType, VNode};

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

impl gxi::Renderable for App {
    fn render(node: &StrongNodeType) {
        gxi! {
            Body::new()
        };

        {
            let cont = node.clone();

            let (node, _) = init_member(&node, InitType::Child, |parent| {
                gxi::Element::from_str("h1", parent).into_vnode_type()
            })
            .unwrap();
            let _ = init_member(&node, InitType::Sibling(&cont), |parent| {
                gxi::Element::from_str("h1", parent).into_vnode_type()
            })
            .unwrap();
        }
    }
}
