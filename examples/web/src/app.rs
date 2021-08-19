use std::ops::DerefMut;

use gxi::Body;
use gxi::{gxi, init_member, InitType, StrongNodeType, VNode};

#[derive(Clone, gxi::Component)]
pub struct App {
    node: gxi::ContainerNode,
}

impl gxi::Renderable for App {
    fn render(__node: &StrongNodeType) {
        gxi! {
            Body
        };

        {
            let cont = __node.clone();

            gxi! {
                h1 ( inner_html = "23", on_click = |_| panic!("hello world"))
            }

            /*{*/
            /*    gxi! {*/
            /*        h1*/
            /*    }*/
            /*}*/
            /*let (node, ..) = init_member(&__node, InitType::Sibling(&cont), |parent| {*/
            /*    gxi::Element::from_str("h1", parent).into_vnode_type()*/
            /*})*/
            /*.unwrap();*/
        }
    }
}
