use std::any::Any;
use std::ops::Deref;
use std::ops::DerefMut;

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
    pub fn manage(self) {
        if self.is_empty() {
            drop(self)
        } else {
            std::mem::forget(self)
        }
    }
}
