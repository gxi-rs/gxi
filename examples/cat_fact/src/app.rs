use serde::{Deserialize, Serialize};

use rust_gui::*;
use rust_gui::Orientation::Vertical;

use crate::centre::Centre;
use crate::counter::Counter;

enum Msg {
    Fetch(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatFact {
    pub length: u32,
    pub fact: String,
}

use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use crate::glib::Sender;
type AsyncState = Arc<Mutex<AppState>>;
pub struct App {
    pub state: AsyncState,
    pub channel_sender: Sender<()>,
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
}
pub struct AppState {
    pub cat_fact: Option<CatFact>
}
impl Node for App {
    fn is_dirty(&self) -> bool {
        self.dirty.clone()
    }
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    fn mark_clean(&mut self) {
        self.dirty = false;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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
    fn add(&mut self, child: NodeRc) {
        let parent = self.parent.upgrade().unwrap();
        parent.as_ref().borrow_mut().add(child);
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
    fn get_type(&self) -> NodeType {
        NodeType::Component
    }

    fn new(parent: WeakNodeRc) -> NodeRc {
        let (channel_sender, re) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            state: Arc::new(Mutex::new(AppState {
                cat_fact: None
            })),
            channel_sender,
            parent,
            dirty: true,
            child: None,
            sibling: None,
        })));
        {
            let this = this.clone();
            re.attach(None, move |_| {
                let this = Rc::clone(&this);

                {
                    let mut node = this.as_ref().borrow_mut();
                    node.mark_dirty();
                }
                Self::render(this);
                glib::Continue(true)
            });
        }
        this
    }
    fn render(this: NodeRc) {
        let cont = Rc::clone(&this);
        let node = cont.clone();
        let state = {
            let mut node_borrow = this.as_ref().borrow_mut();
            let node = node_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
            if !node.is_dirty() { return; }
            node.mark_clean();
            node.state.clone()
        };
        let state = state.lock().unwrap();
        let node = {
            let (node, is_new) = {
                {}
                let mut node_borrow = node.as_ref().borrow_mut();
                let weak_cont = Rc::downgrade(&cont);
                node_borrow.init_child(Box::new(move || Init::new(weak_cont)))
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<Init>().unwrap();
                if is_new {
                    {
                        let state_clone = Rc::clone(&this);
                        node.on_init(move || Self::update(state_clone.clone(), Msg::Fetch(true)));
                    }
                }
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
                    node_borrow.init_child(Box::new(move || View::new(weak_cont)))
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                    if is_new {}
                    node.orientation(Vertical);
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
                        node_borrow.init_child(Box::new(move || Centre::new(weak_cont)))
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<Centre>().unwrap();
                        if is_new {}
                    }
                    node
                };
                Centre::render(node.clone());
                {
                    let cont = node.clone();
                    let node = {
                        let (node, is_new) = {
                            {}
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let cont = node_borrow.as_any_mut().downcast_mut::<Centre>().unwrap();
                            let cont = cont.parent_substitute.clone();
                            let cont_borrow = cont.upgrade().unwrap();
                            let mut cont_borrow = cont_borrow.borrow_mut();
                            cont_borrow.init_child(Box::new(move || Image::new(cont)))
                        };
                        {
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let node = node_borrow.as_any_mut().downcast_mut::<Image>().unwrap();
                            if is_new { node.source("cat.gif"); }
                        }
                        node
                    };
                    Image::render(node.clone());
                }
                Centre::render(node.clone());
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
                            {
                                let state_clone = Rc::clone(&this);
                                node.on_click(move || Self::update(state_clone.clone(), Msg::Fetch(false)));
                            }
                            node.label("Fetch Cat Memes");
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
                    let (node, is_new) = {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_child(Box::new(move || Pure::new(weak_cont)))
                    };
                    {
                        let cont = node.clone();
                        if state.cat_fact.is_none() {
                            let node = {
                                let (node, is_new) = {
                                    {
                                        let pure_index = {
                                            let mut node_borrow = node.as_ref().borrow_mut();
                                            let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                                            let index = pure.pure_index.clone();
                                            pure.pure_index = 1u32;
                                            index
                                        };
                                        if pure_index != 1u32 { let node = { node.as_ref().borrow_mut().get_child_mut().take() }; }
                                    }
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
                            {
                                let cont = node.clone();
                                let node = {
                                    let (node, is_new) = {
                                        {}
                                        let mut node_borrow = node.as_ref().borrow_mut();
                                        let weak_cont = Rc::downgrade(&cont);
                                        node_borrow.init_child(Box::new(move || Text::new(weak_cont)))
                                    };
                                    {
                                        let mut node_borrow = node.as_ref().borrow_mut();
                                        let node = node_borrow.as_any_mut().downcast_mut::<Text>().unwrap();
                                        if is_new { node.label("loading"); }
                                    }
                                    node
                                };
                                Text::render(node.clone());
                                let node = {
                                    let (node, is_new) = {
                                        {}
                                        let mut node_borrow = node.as_ref().borrow_mut();
                                        let weak_cont = Rc::downgrade(&cont);
                                        node_borrow.init_sibling(Box::new(move || Spinner::new(weak_cont)))
                                    };
                                    {
                                        let mut node_borrow = node.as_ref().borrow_mut();
                                        let node = node_borrow.as_any_mut().downcast_mut::<Spinner>().unwrap();
                                        if is_new { node.spin(true); }
                                    }
                                    node
                                };
                                Spinner::render(node.clone());
                            }
                        } else {
                            let node = {
                                let (node, is_new) = {
                                    {
                                        let pure_index = {
                                            let mut node_borrow = node.as_ref().borrow_mut();
                                            let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                                            let index = pure.pure_index.clone();
                                            pure.pure_index = 2u32;
                                            index
                                        };
                                        if pure_index != 2u32 { let node = { node.as_ref().borrow_mut().get_child_mut().take() }; }
                                    }
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let weak_cont = Rc::downgrade(&cont);
                                    node_borrow.init_child(Box::new(move || Text::new(weak_cont)))
                                };
                                {
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let node = node_borrow.as_any_mut().downcast_mut::<Text>().unwrap();
                                    if is_new {}
                                    node.label(&state.cat_fact.as_ref().unwrap().fact);
                                }
                                node
                            };
                        }
                    }
                }
                View::render(node.clone());
                let node = {
                    let (node, is_new) = {
                        {}
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_sibling(Box::new(move || Counter::new(weak_cont)))
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<Counter>().unwrap();
                        if is_new {}
                        node.count(if let Some(cat_fact) = &state.cat_fact { Some(cat_fact.length) } else { None });
                    }
                    node
                };
                Counter::render(node.clone());
            }
            View::render(node.clone());
        }
        Init::render(node.clone());
    }
}
impl Drop for App {
    fn drop(&mut self) {}
}
#[update(App)]
async fn update<F: Fn() + 'static>(state: AsyncState, msg: Msg, render: F) -> AsyncResult<ShouldRender> {
    match msg {
        Msg::Fetch(force) => {
            if {
                let mut state = state.lock().unwrap();
                if state.cat_fact.is_some() {
                    state.cat_fact = None;
                    render();
                    true
                } else { false }
            } || force {
                let resp = reqwest::get("https://catfact.ninja/fact?max_length=140").await?;
                let cat_fact = resp.json::<CatFact>().await?;
                let mut state = state.lock().unwrap();
                state.cat_fact = Some(cat_fact);
                Ok(ShouldRender::Yes)
            } else {
                Ok(ShouldRender::No)
            }
        }
    }
}
