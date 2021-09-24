use gxi::{gxi, Body, StrongNodeType};

fn use_state<T>(v: T) -> gxi::Observable<T> {
    gxi::Observable::new(v)
}

pub fn app() -> StrongNodeType {
    let _counter = use_state(0);
    let _const_value = "yp";
    // add this to gxi macro
    return gxi! {
        Body [
           button ( /*on_click = |_| panic!("hello"),*/ inner_html = "click me"),
           p ( inner_html = "yp" )
        ]
    };
}
