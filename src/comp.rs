enum Node {
    Widget(Box<dyn element::Element>),
    Container(Box<dyn container::Container>),
    Component(Box<dyn component::Component>),
}

mod container {
    use crate::comp::element::Element;

    pub trait Container {}
}

mod element {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::comp::container::Container;

    pub trait Element {}
}

mod component {
    pub trait Component {}
}

mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::comp::Node;
    use crate::comp::container::Container;

    mod views {
        use std::cell::RefCell;
        use std::rc::Rc;

        use crate::comp::container::Container;
        use crate::comp::element::Element;

        pub struct Grid {}

        impl Container for Grid {}
    }

    mod widgets {
        use std::cell::RefCell;
        use std::rc::Rc;

        use crate::comp::container::Container;
        use crate::comp::element::Element;

        pub struct Button {
            pub parent: Rc<RefCell<Box<dyn Container>>>
        }

        impl Element for Button {
        }
    }

    #[test]
    fn test() {
        let container = Rc::new(RefCell::new(Box::new(views::Grid {})));
        let _bt = Box::new(widgets::Button {
            parent: container.clone()
        });
    }
}
