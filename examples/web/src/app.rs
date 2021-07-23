use gxi::InitType;
use gxi::StrongNodeType;
use gxi::VNode;
use gxi::init_member;

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

impl gxi::Renderable for App {
    fn render(&mut self) {
        let mut node_ref = self.node.as_ref().borrow_mut();
        
        let node = self
            .into_vnode_type()
            .init_member(InitType::Child, || gxi::Body::default().into_vnode_type())
            .unwrap();
        
        let _node = node
            .init_member(InitType::Child, || gxi::WebElement::from("h1").into().into_vnode_type()
            })
            .unwrap();
    }
}

