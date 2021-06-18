use crate::*;

gxi! {
    pub Comp {
        pub class : &'static str = "",
        pub id : &'static str = ""
    }
    render {
        Pure [
            #children
        ]
    }
}

impl Comp {
    pub fn from_class(parent: WeakNodeType, class: &'static str) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            state: Rc::new(RefCell::new(CompState { class, id: "" })),
            self_substitute: None,
            parent,
            is_dirty: true,
            child: None,
            sibling: None,
        }))))
    }
}
