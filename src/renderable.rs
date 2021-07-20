use crate::StrongNodeType;

pub trait Renderable {
    fn render(this: &StrongNodeType) {}
}
