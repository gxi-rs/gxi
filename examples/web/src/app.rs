use gxi::{gxi, set_state, use_state, Body, StrongNodeType};
//
pub fn app() -> StrongNodeType {
    let h1_value = "hello";
    let reduce_emoji = use_state("🙃");
    
    // add this to gx
    return gxi! {
        Body [
           h1 ( const inner_html = h1_value ),
           Counter::new(2, reduce_emoji.clone()),
           Counter::new(20, reduce_emoji.clone())
        ]
    };
}

#[gxi::comp]
pub fn Counter(initial: i32, mut reduce_emoji: gxi::Observable<&'static str>) -> StrongNodeType {
    let mut counter = use_state(initial);

    return gxi! {
        div [
            h1 ( inner_html = &counter.to_string()[..] ),
            div [
                button ( on_click = set_state!( counter, *counter + 1), inner_html = "+" ),
                button ( on_click = set_state!( counter, *counter - 1), inner_html = &reduce_emoji.to_string()[..] )
            ]
        ]
    };
}
