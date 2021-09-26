use crate::{StrongNodeType, WeakNodeType};

pub trait Renderable {
    fn render(_this: &StrongNodeType) {}
}

pub trait StatefulNode {
    type Msg;
    type State;

    fn update(__this: &WeakNodeType, __msg: Self::Msg) {}
}
