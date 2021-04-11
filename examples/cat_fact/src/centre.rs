use rust_gui::*;

use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use crate::glib::Sender;
type AsyncState = Arc<Mutex<CentreState>>;
pub struct Centre {
    pub state: AsyncState,
    pub channel_sender: Sender<()>,
    pub parent: WeakNodeRc,
    pub parent_substitute : WeakNodeRc,
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
}
pub struct CentreState {
    pub cat_fact: Option<String>
}
impl Node for Centre {
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
            state: Arc::new(Mutex::new(CentreState {
                cat_fact: None
            })),
            channel_sender,
            parent_substitute : parent.clone(),
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
                    node_borrow.init_child(Box::new(move || View::new(weak_cont)))
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                    if is_new { node.v_expand(true); }
                }
                node
            };
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
                    let (node, is_new) = {
                        {}
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_child(Box::new(move || View::new(weak_cont)))
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                        if is_new { node.h_expand(true); }
                    }
                    node
                };
                View::render(node.clone());
                let node = {
                    let (node, is_new) = {
                        {}
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_sibling(Box::new(move || Pure::new(weak_cont)))
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
                    let mut this_borrow = this.as_ref().borrow_mut();
                    let this =  this_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                    this.parent_substitute = Rc::downgrade(&cont);
                }
                Pure::render(node.clone());
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
                        if is_new { node.h_expand(true); }
                    }
                    node
                };
                View::render(node.clone());
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
                    if is_new { node.v_expand(true); }
                }
                node
            };
            View::render(node.clone());
        }
        View::render(node.clone());
    }
}
impl Drop for Centre {
    fn drop(&mut self) {}
}