pub enum Node {
    /// widget which cannot contain other nodes
    Leaf,
    /// widget which can hold other widgets
    Container,
    /// widget which can hold other widgets but can't be added to other nodes
    TopLevelContainer,
}

//impl<'a> Deref for Node<'a> {
//    type Target = dyn VNode;
//
//    fn deref(&self) -> &Self::Target {
//        match self {
//            Node::Leaf(node) => node.deref().as_ref(),
//            Node::TopLevelContainer(node) => node.deref().as_ref(),
//            Node::Container(node) => node.deref().as_ref(),
//        }
//    }
//}
//
//impl<'a> DerefMut for Node<'a> {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        match self {
//            Node::Leaf(node) => node.deref_mut().as_mut(),
//            Node::Container(node) => node.deref_mut().as_mut(),
//            Node::TopLevelContainer(node) => node.deref_mut().as_mut(),
//        }
//    }
//}

//impl<'a> Node<'a> {
//    pub fn get_native_widget(&self) -> &NativeWidget {
//        match self {
//            Node::Leaf(w) => w.deref(),
//            Node::Container(w) => w.deref(),
//            Node::TopLevelContainer(w) => w.deref(),
//        }
//    }
//}
