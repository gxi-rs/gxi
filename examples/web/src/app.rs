use gxi::InitType;
use gxi::StrongNodeType;
use gxi::VNode;
use gxi::WeakNodeType;
use gxi::init_member;

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

impl gxi::Renderable for App {
    fn render(this: &StrongNodeType) {
        let node = init_member(this, InitType::Child, |parent| gxi::Body::new(parent).into_vnode_type()).unwrap();

        let _node = node.init_member(&node, InitType::Child, || {
                gxi::WebElement::from("h1").into().into_vnode_type()
            }).unwrap();
    }
}
