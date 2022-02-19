use crate::{cat_fact, complex_counter};
use gxi::{gxi, Body, VNodeContext};

pub fn app() -> VNodeContext<Body> {
    gxi! {
        Body [
            complex_counter(),
            cat_fact(),
            //todo()
        ]
    }
}
