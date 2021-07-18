use crate::VNodeType;

/// node of a n-binary tree which can hold a child and a sibling
pub enum Node {
    Container(ContainerNode),
    Widget(WidgetNode)
}

/// node of a n-binary tree which can hold a child and a sibling
#[derive(Default)]
pub struct ContainerNode {
    pub child: Option<VNodeType>,
    pub sibling: Option<VNodeType>,
}

/// node of a n-binary tree which can hold only a sibling
#[derive(Default)]
pub struct WidgetNode {
    pub sibling: Option<VNodeType>,
}

impl Node {
    pub fn get_child_mut(&mut self) -> Result<&mut Option<VNodeType>, &'static str> {
        match self {
            Node::Container(container) => {
                Ok(&mut container.child)
            }
            Node::Widget(widget) => {
                Err("This widget can't contain a child")
            }
        }
    }
    
}
