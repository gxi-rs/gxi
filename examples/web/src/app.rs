use crate::{cat_fact, comlex_counter};
use gxi::{gxi, Body, StrongNodeType};

pub fn app() -> StrongNodeType {
    unsafe {
        return gxi! {
            Body [
                comlex_counter(),
                cat_fact()
            ]
        };
    }
}
