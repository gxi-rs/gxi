use gxi::{gxi, Body, StrongNodeType}; use std::ops::Deref;
//
fn use_state<T>(v: T) -> gxi::Observable<T> {
    gxi::Observable::new(v)
}
//
pub fn app() -> StrongNodeType {
    let mut counter = use_state(0);
    let _const_value = "yp";
    // add this to gx
    let mut counter2 = counter.clone();
    return gxi! {
        Body [
           button ( on_click = move |_| counter2.set_value(*counter2 + 1), inner_html = "click me"),
           p ( inner_html = &(counter.deref().to_string())[..] )
        ]
    };
}
