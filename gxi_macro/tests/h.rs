mod comps {
    mod comp {
        use crate::*;
        use gxi::get_rc_state as get_state;
        use gxi::get_mut_rc_state as get_state_mut;
        use std::any::Any;
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Mutex, Arc};
        use std::ops::DerefMut;
        type State = Rc<RefCell<CompState>>;
        pub struct CompState {
            class: String,
            id: String,
        }
        pub struct Comp {
            state: State,
            pub parent: WeakNodeType,
            pub self_substitute: Option<WeakNodeType>,
            pub is_dirty: bool,
            pub child: Option<StrongNodeType>,
            pub sibling: Option<StrongNodeType>,
        }
        impl Node for Comp {
            fn as_node(&self) -> &dyn Node {
                self
            }
            fn as_node_mut(&mut self) -> &mut dyn Node {
                self
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn get_parent(&self) -> &WeakNodeType {
                &self.parent
            }
            fn get_sibling(&self) -> &Option<StrongNodeType> {
                &self.sibling
            }
            fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
                &mut self.sibling
            }
            fn new(parent: WeakNodeType) -> StrongNodeType {
                let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
                    state: Rc::new(RefCell::new(CompState {
                        class: "".to_string(),
                        id: "".to_string(),
                    })),
                    self_substitute: None,
                    parent,
                    is_dirty: true,
                    child: None,
                    sibling: None,
                }))));
                this
            }
            fn render(this: StrongNodeType) {
                let node = Rc::clone(&this);
                let state = {
                    let mut node = this.as_ref().borrow_mut();
                    let node = node
                        .as_node_mut()
                        .as_any_mut()
                        .downcast_mut::<Self>()
                        .unwrap();
                    if !node.is_dirty() {
                        return;
                    }
                    node.mark_clean();
                    node.state.clone()
                };
                let state = state.as_ref().borrow();
                let node = {
                    let (node, ..) =
                        init_member(node.clone(), InitType::Child, |this| Pure::new(this));
                    {
                        let mut this_borrow = this.as_ref().borrow_mut();
                        match this_borrow.deref_mut() {
                            GxiNodeType::Component(t) => {
                                *t.get_self_substitute_mut() = Some(Rc::downgrade(&node))
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                    }
                    node
                };
            }
        }
        impl Container for Comp {
            fn get_child(&self) -> &Option<StrongNodeType> {
                &self.child
            }
            fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
                &mut self.child
            }
            fn as_container(&self) -> &dyn Container {
                self
            }
            fn as_container_mut(&mut self) -> &mut dyn Container {
                self
            }
        }
        impl ComponentNode for Comp {
            fn get_self_substitute(&self) -> &Option<WeakNodeType> {
                &self.self_substitute
            }
            fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
                &mut self.self_substitute
            }
            fn is_dirty(&self) -> bool {
                self.is_dirty
            }
            fn mark_dirty(&mut self) {
                self.is_dirty = true
            }
            fn mark_clean(&mut self) {
                self.is_dirty = false
            }
        }
        impl Comp {
            pub fn class(&mut self, val: String) {
                if {
                    let mut state = self.state.as_ref().borrow_mut();
                    if val != state.class {
                        state.class = val;
                        true
                    } else {
                        false
                    }
                } {
                    self.mark_dirty();
                }
            }
            pub fn id(&mut self, val: String) {
                if {
                    let mut state = self.state.as_ref().borrow_mut();
                    if val != state.id {
                        state.id = val;
                        true
                    } else {
                        false
                    }
                } {
                    self.mark_dirty();
                }
            }
        }
    }
    mod foo {
        use gxi::*;
        use gxi::get_rc_state as get_state;
        use gxi::get_mut_rc_state as get_state_mut;
        use std::any::Any;
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Mutex, Arc};
        use std::ops::DerefMut;
        type State = Rc<RefCell<FooState>>;
        pub struct FooState {}
        pub struct Foo {
            state: State,
            pub parent: WeakNodeType,
            pub self_substitute: Option<WeakNodeType>,
            pub is_dirty: bool,
            pub child: Option<StrongNodeType>,
            pub sibling: Option<StrongNodeType>,
        }
        impl Node for Foo {
            fn as_node(&self) -> &dyn Node {
                self
            }
            fn as_node_mut(&mut self) -> &mut dyn Node {
                self
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn get_parent(&self) -> &WeakNodeType {
                &self.parent
            }
            fn get_sibling(&self) -> &Option<StrongNodeType> {
                &self.sibling
            }
            fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
                &mut self.sibling
            }
            fn new(parent: WeakNodeType) -> StrongNodeType {
                let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
                    state: Rc::new(RefCell::new(FooState {})),
                    self_substitute: None,
                    parent,
                    is_dirty: true,
                    child: None,
                    sibling: None,
                }))));
                this
            }
        }
        impl Container for Foo {
            fn get_child(&self) -> &Option<StrongNodeType> {
                &self.child
            }
            fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
                &mut self.child
            }
            fn as_container(&self) -> &dyn Container {
                self
            }
            fn as_container_mut(&mut self) -> &mut dyn Container {
                self
            }
        }
        impl ComponentNode for Foo {
            fn get_self_substitute(&self) -> &Option<WeakNodeType> {
                &self.self_substitute
            }
            fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
                &mut self.self_substitute
            }
            fn is_dirty(&self) -> bool {
                self.is_dirty
            }
            fn mark_dirty(&mut self) {
                self.is_dirty = true
            }
            fn mark_clean(&mut self) {
                self.is_dirty = false
            }
        }
        impl Foo {}
    }
    pub use comp::*;
    pub use foo::*;
}
mod helpers {
    use gxi::*;
    pub fn no_siblibng(node: StrongNodeType) {
        let node_borrow = node.as_ref().borrow();
        if node_borrow.as_node().get_sibling().is_some() {
            {
                panic!("no sibling was expected")
            };
        }
    }
    pub fn check_child_type<T: 'static + Node>(node: StrongNodeType, name: &str) -> StrongNodeType {
        let node_borrow = node.as_ref().borrow();
        let child = node_borrow
            .as_container()
            .unwrap()
            .get_child()
            .as_ref()
            .unwrap()
            .clone();
        let child_borrow = child.as_ref().borrow();
        child_borrow
            .as_node()
            .as_any()
            .downcast_ref::<T>()
            .expect("");
        drop(child_borrow);
        child
    }
    pub fn check_sibling_type<T: 'static + Node>(
        node: StrongNodeType,
        name: &str,
    ) -> StrongNodeType {
        let node_borrow = node.as_ref().borrow();
        let sibling = node_borrow
            .as_node()
            .get_sibling()
            .as_ref()
            .unwrap()
            .clone();
        let sibling_borrow = sibling.as_ref().borrow();
        sibling_borrow
            .as_node()
            .as_any()
            .downcast_ref::<T>()
            .expect("");
        drop(sibling_borrow);
        sibling
    }
}
pub use comps::*;
pub use helpers::*;
pub use gxi::*;
use gxi::get_rc_state as get_state;
use gxi::get_mut_rc_state as get_state_mut;
use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::ops::DerefMut;
type State = Rc<RefCell<AppState>>;
struct AppState {
    limit: u32,
}
struct App {
    state: State,
    pub parent: WeakNodeType,
    pub self_substitute: Option<WeakNodeType>,
    pub is_dirty: bool,
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
}
impl Node for App {
    fn as_node(&self) -> &dyn Node {
        self
    }
    fn as_node_mut(&mut self) -> &mut dyn Node {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_parent(&self) -> &WeakNodeType {
        &self.parent
    }
    fn get_sibling(&self) -> &Option<StrongNodeType> {
        &self.sibling
    }
    fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
        &mut self.sibling
    }
    fn new(parent: WeakNodeType) -> StrongNodeType {
        let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            state: Rc::new(RefCell::new(AppState { limit: 0 })),
            self_substitute: None,
            parent,
            is_dirty: true,
            child: None,
            sibling: None,
        }))));
        this
    }
    fn render(this: StrongNodeType) {
        let node = Rc::clone(&this);
        let state = {
            let mut node = this.as_ref().borrow_mut();
            let node = node
                .as_node_mut()
                .as_any_mut()
                .downcast_mut::<Self>()
                .unwrap();
            if !node.is_dirty() {
                return;
            }
            node.mark_clean();
            node.state.clone()
        };
        let state = state.as_ref().borrow();
        let node = {
            let (node, is_new) = init_member(node.clone(), InitType::Child, |this| {
                crate::comps::Comp::new(this)
            });
            crate::comps::Comp::render(node.clone());
            get_substitute(node)
        };
        {
            let (node, ..) = init_member(node.clone(), InitType::Child, |this| Pure::new(this));
            {
                let cont = node.clone();
                if state.limit == 0 {
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let pure = node_borrow
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<Pure>()
                            .unwrap();
                        if pure.pure_index != 1u32 {
                            pure.pure_index = 1u32;
                            pure.child = None;
                        }
                    }
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Child, |this| Comp::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<Comp>()
                                .unwrap();
                            node.class("h1".to_string());
                            node.id("asd".to_string());
                        }
                        Comp::render(node.clone());
                        get_substitute(node)
                    };
                    {
                        let node = {
                            let (node, is_new) =
                                init_member(node.clone(), InitType::Child, |this| Comp::new(this));
                            Comp::render(node.clone());
                            get_substitute(node)
                        };
                        {}
                        let node = {
                            let (node, is_new) =
                                init_member(node.clone(), InitType::Sibling, |this| {
                                    Comp::new(this)
                                });
                            Comp::render(node.clone());
                            node
                        };
                        {}
                        let (node, ..) =
                            init_member(node.clone(), InitType::Sibling, |this| Pure::new(this));
                        {
                            let (node, ..) =
                                init_member(node.clone(), InitType::Child, |this| Pure::new(this));
                            let mut prev_sibling = node.clone();
                            for x in 0..2 {
                                let node = prev_sibling.clone();
                                let node = {
                                    let (node, is_new) =
                                        init_member(node.clone(), InitType::Sibling, |this| {
                                            Comp::new(this)
                                        });
                                    Comp::render(node.clone());
                                    node
                                };
                                {
                                    let node = {
                                        let (node, is_new) =
                                            init_member(node.clone(), InitType::Child, |this| {
                                                Comp::new(this)
                                            });
                                        Comp::render(node.clone());
                                        get_substitute(node)
                                    };
                                    {}
                                }
                                prev_sibling = node;
                            }
                            *prev_sibling
                                .as_ref()
                                .borrow_mut()
                                .as_node_mut()
                                .get_sibling_mut() = None;
                        }
                    }
                } else {
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let pure = node_borrow
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<Pure>()
                            .unwrap();
                        if pure.pure_index != 2u32 {
                            pure.pure_index = 2u32;
                            pure.child = None;
                        }
                    }
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Child, |this| Foo::new(this));
                        Foo::render(node.clone());
                        get_substitute(node)
                    };
                    {}
                }
            }
            let node = {
                let (node, is_new) =
                    init_member(node.clone(), InitType::Sibling, |this| Comp::new(this));
                Comp::render(node.clone());
                node
            };
            {}
        }
    }
}
impl Container for App {
    fn get_child(&self) -> &Option<StrongNodeType> {
        &self.child
    }
    fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
        &mut self.child
    }
    fn as_container(&self) -> &dyn Container {
        self
    }
    fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
}
impl ComponentNode for App {
    fn get_self_substitute(&self) -> &Option<WeakNodeType> {
        &self.self_substitute
    }
    fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
        &mut self.self_substitute
    }
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    fn mark_dirty(&mut self) {
        self.is_dirty = true
    }
    fn mark_clean(&mut self) {
        self.is_dirty = false
    }
}
impl App {}
