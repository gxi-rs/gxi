use std::ops::Deref;

use crate::{self as gxi, StrongNodeType, VNodeType};
use gxi_derive::gxi_vnode;

/// Special Container acting as a bridge between 2 nodes
#[gxi_vnode(PlaceHolderContainer)]
#[derive(Default)]
pub struct PlaceHolder {
    value: Option<StrongNodeType>,
}

impl PlaceHolder {
    pub fn get_value(&self) -> &Option<StrongNodeType> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Option<StrongNodeType> {
        &mut self.value
    }

    /// get node which is not PlaceHolderContainer
    pub fn get_nested_not_place_holder_node(&self) -> Option<StrongNodeType> {
        if let Some(value) = &self.value {
            if let VNodeType::PlaceHolderContainer(value) = value.as_ref().borrow().deref() {
                return value.get_nested_not_place_holder_node();
            } else {
                return Some(value.clone());
            }
        } else {
            return None;
        }
    }
}
