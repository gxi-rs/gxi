use gxi::init_member;
use gxi::InitType;
use gxi::StrongNodeType;
use gxi::VNode;

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

impl gxi::Renderable for App {
    fn render(this: &StrongNodeType) {
        let (node, _) = init_member(this, InitType::Child, |parent| {
            gxi::Body::new(parent).into_vnode_type()
        })
        .unwrap();

        let _node = init_member(&node, InitType::Child, |parent| {
            gxi::WebContainer::from_str("h1", parent).into_vnode_type()
        })
        .unwrap();
    }
}
