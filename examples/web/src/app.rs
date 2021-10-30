use crate::{cat_fact, complex_counter, todo};
use gxi::{gxi, Body, StrongNodeType};

pub fn app() -> StrongNodeType {
    return gxi! {
        Body [
            complex_counter(),
            cat_fact(),
            todo()
        ]
    };
}
