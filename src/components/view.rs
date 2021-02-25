use crate::components::Component;

pub struct View {
    pub sibling: Option<Box<dyn Component>>,
    pub child: Option<Box<dyn Component>>,
}

impl Default for View {
    fn default() -> Self {
        View {
            sibling: None,
            child: None,
        }
    }
}

impl Component for View {
    fn get_sibling(&self) -> Option<&Box<dyn Component>> {
        self.sibling.as_ref()
    }

    fn get_sibling_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        self.sibling.as_mut()
    }

    fn get_child(&self) -> Option<&Box<dyn Component>> {
        self.child.as_ref()
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        self.child.as_mut()
    }
}