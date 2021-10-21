use crate::{cat_fact::CatFact, ComplexCounter};
use gxi::{gxi, Body, StrongNodeType};

pub fn app() -> StrongNodeType {
    unsafe {
        return gxi! {
            Body [
                ComplexCounter,
                CatFact
            ]
        };
    }
}
