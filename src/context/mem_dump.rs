use std::any::Any;
use std::ops::Deref;
use std::ops::DerefMut;

/// holds an arbitrary amount of memory in a memory pool
#[derive(Default)]
pub struct MemDump(Vec<Box<dyn Any>>);

impl Deref for MemDump {
    type Target = Vec<Box<dyn Any>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MemDump {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MemDump {
    /// consumes self and leaks memory pool
    pub fn leak(self) {
        if self.is_empty() {
            drop(self)
        } else {
            std::mem::forget(self)
        }
    }
}
