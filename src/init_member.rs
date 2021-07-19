use crate::VNodeType;

pub enum InitType {
    Child,
    Sibling,
}

impl VNodeType {
    pub fn init_member<C: FnOnce() -> VNodeType>(
        &mut self,
        init_type: InitType,
        init: C,
    ) -> Result<&mut VNodeType, &'static str> {
        match init_type {
            InitType::Child => {
                let node = match self {
                    VNodeType::Component(comp) => comp.get_node_ref().borrow_mut(),
                    VNodeType::Widget(widget) => widget.get_node(),
                    VNodeType::TopLevelWidget(top) => top.get_node(),
                };
                let child = node.get_child_mut()?;
                Ok(child.get_or_insert_with(init))
            }
            InitType::Sibling => {
                todo!()
            }
        }
    }
}
