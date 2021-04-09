use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{FutureExt, TryFutureExt};
use tokio::*;
use tokio::prelude::*;
use tokio::prelude::*;
use tokio::task::LocalSet;

use rust_gui::*;
use rust_gui::gtk::DialogExt;

enum Msg {
    INC,
    DEC,
}

pub struct MyApp {
    pub state: AsyncState<MyAppState>,
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: gtk::Container,
}

pub struct MyAppState {
    pub count: u32,
}

impl Node for MyApp {
    fn get_child(&self) -> &Option<NodeRc> {
        &self.child
    }
    fn get_child_mut(&mut self) -> &mut Option<NodeRc> {
        &mut self.child
    }
    fn get_sibling(&self) -> &Option<NodeRc> {
        &self.sibling
    }
    fn get_sibling_mut(&mut self) -> &mut Option<NodeRc> {
        &mut self.sibling
    }
    fn init_child(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        match self.child {
            None => {
                let child = self.child.get_or_insert(f());
                if let NodeType::Widget = child.as_ref().borrow().get_type() {
                    let parent = self.parent.upgrade().unwrap();
                    parent.as_ref().borrow_mut().add(child.clone());
                }
                (child.clone(), true)
            }
            _ => (self.child.as_ref().unwrap().clone(), false),
        }
    }
    fn init_sibling(&mut self, f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        match self.sibling {
            None => {
                let sibling = self.sibling.get_or_insert(f());
                if let NodeType::Widget = sibling.as_ref().borrow().get_type() {
                    let parent = self.parent.upgrade().unwrap();
                    parent.as_ref().borrow_mut().add(sibling.clone());
                }
                (sibling.clone(), true)
            }
            _ => (self.sibling.as_ref().unwrap().clone(), false),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_widget(&self) -> gtk::Widget {
        let parent = self.parent.upgrade().unwrap();
        let parent = parent.as_ref().borrow();
        parent.get_widget()
    }
    fn get_widget_as_container(&self) -> gtk::Container {
        let parent = self.parent.upgrade().unwrap();
        let parent = parent.as_ref().borrow();
        parent.get_widget_as_container()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Component
    }
    fn new(parent: WeakNodeRc) -> NodeRc {
        Rc::new(RefCell::new(Box::new(Self {
            state: Arc::new(Mutex::new(MyAppState { count: 0 })),
            widget: parent
                .upgrade()
                .unwrap()
                .as_ref()
                .borrow()
                .get_widget_as_container(),
            parent,
            dirty: true,
            child: None,
            sibling: None,
        })))
    }
    fn render(this: NodeRc) {
        let cont = Rc::clone(&this);
        let node = cont.clone();
        let state = {
            let state_borrow = this.as_ref().borrow();
            let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
            state.state.clone()
        };
        let state = state.lock().unwrap();
        let node = {
            let (node, is_new) = {
                {}
                let mut node_borrow = node.as_ref().borrow_mut();
                let weak_cont = Rc::downgrade(&cont);
                node_borrow.init_child(Box::new(move || View::new(weak_cont)))
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                if is_new {}
            }
            node
        };
        {
            let cont = node.clone();
            let node = {
                let (node, is_new) = {
                    {}
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.init_child(Box::new(move || Image::new(weak_cont)))
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<Image>().unwrap();
                    if is_new {
                        node.source("cat.gif");
                    }
                }
                node
            };
            Image::render(node.clone());
            let node = {
                let (node, is_new) = {
                    {}
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.init_sibling(Box::new(move || View::new(weak_cont)))
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                    if is_new {}
                }
                node
            };
            {
                let cont = node.clone();
                let node = {
                    let (node, is_new) = {
                        {}
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_child(Box::new(move || Button::new(weak_cont)))
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<Button>().unwrap();
                        if is_new {
                            node.label("Inc");
                            {
                                let state_clone = Rc::clone(&this);
                                node.on_click(move |_| {
                                    Self::update(state_clone.clone(), Msg::INC)
                                });
                            }
                        }
                    }
                    node
                };
                Button::render(node.clone());
                let node = {
                    let (node, is_new) = {
                        {}
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_sibling(Box::new(move || Button::new(weak_cont)))
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<Button>().unwrap();
                        if is_new {
                            node.label("Dec");
                            {
                                let state_clone = Rc::clone(&this);
                                node.on_click(move |_| {
                                    Self::update(state_clone.clone(), Msg::DEC)
                                });
                            }
                        }
                    }
                    node
                };
                Button::render(node.clone());
            }
            View::render(node.clone());
            let node = {
                let (node, is_new) = {
                    {}
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.init_sibling(Box::new(move || View::new(weak_cont)))
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                    if is_new {}
                }
                node
            };
            {
                let cont = node.clone();
                let node = {
                    let node = {
                        let (node, is_new) = {
                            {}
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let weak_cont = Rc::downgrade(&cont);
                            node_borrow.init_child(Box::new(move || Pure::new(weak_cont)))
                        };
                        {
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let node = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                            if is_new {}
                        }
                        node
                    };
                    Pure::render(node.clone());
                    {
                        let cont = node.clone();
                        let node = {
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let weak_cont = Rc::downgrade(&cont);
                            node_borrow
                                .init_child(Box::new(move || Pure::new(weak_cont)))
                                .0
                        };
                        let cont = node.clone();
                        let mut prev_node = node.clone();
                        for x in 0..state.count {
                            let node = prev_node.clone();
                            let node = {
                                let (node, is_new) = {
                                    {}
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let weak_cont = Rc::downgrade(&cont);
                                    node_borrow
                                        .init_sibling(Box::new(move || Text::new(weak_cont)))
                                };
                                {
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let node = node_borrow
                                        .as_any_mut()
                                        .downcast_mut::<Text>()
                                        .unwrap();
                                    if is_new {}
                                    node.label(&x.to_string());
                                }
                                node
                            };
                            Text::render(node.clone());
                            prev_node = node;
                        }
                        {
                            prev_node.as_ref().borrow_mut().get_sibling_mut().take();
                        }
                    }
                    node
                };
            }
            View::render(node.clone());
            let node = {
                let (node, is_new) = {
                    {}
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.init_sibling(Box::new(move || Button::new(weak_cont)))
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<Button>().unwrap();
                    if is_new {
                        node.label("Last");
                    }
                }
                node
            };
            Button::render(node.clone());
        }
        View::render(node.clone());
    }
    fn is_dirty(&self) -> bool {
        self.dirty.clone()
    }
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    fn mark_clean(&mut self) {
        self.dirty = false;
    }
    fn add(&mut self, child: NodeRc) {
        let parent = self.parent.upgrade().unwrap();
        parent.as_ref().borrow_mut().add(child);
    }
}

impl MyApp {
    fn update(this: NodeRc, msg: Msg) {
        async fn update_logic(state: Arc<Mutex<MyAppState>>, msg: Msg) -> ShouldRender {
            {
                println!("sleeping for 60 secs");
                time::delay_for(Duration::from_secs(1)).await;
                let mut state = state.lock().unwrap();
                println!("slept for 60 secs");
                match msg {
                    Msg::INC => {
                        state.count += 1;
                    }
                    _ => {
                        if state.count > 0 {
                            state.count -= 1;
                        } else {
                            return ShouldRender::No;
                        }
                    }
                }
                ShouldRender::Yes
            }
        }

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let state = {
            let state_borrow = this.as_ref().borrow();
            let state = state_borrow.as_any().downcast_ref::<MyApp>().unwrap();
            state.state.clone()
        };

        task::spawn(async move {
            let should_render = update_logic(state, msg).await;
            if let ShouldRender::Yes = should_render {
                tx.send(()).unwrap();
            }
        });

        let this_clone = this.clone();
        rx.attach(None, move |_| {
            Self::render(Rc::clone(&this_clone));
            glib::Continue(true)
        });
    }
}

impl Drop for MyApp {
    fn drop(&mut self) {}
}