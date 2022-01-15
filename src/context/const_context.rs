use crate::MemDump;

/// A RefCell wrapper around MemDump
/// Used for nodes which live throughout the
/// lifetime of the component
pub type ConstContextNode = MemDump;
//
//#[derive(Default)]
//pub struct ConstContextNode(MemDump);
//
//impl Deref for ConstContextNode {
//    type Target = RefCell<MemDump>;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
