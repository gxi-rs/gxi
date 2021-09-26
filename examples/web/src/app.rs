use gxi::{gxi, set_state, use_state, Body, StrongNodeType};

const EMOTICONS: [&'static str; 3] = ["-", "🙃", "|"];

pub fn app() -> StrongNodeType {
    let h1_value = "hello";
    let reduce_emoji = use_state(EMOTICONS[0]);
    let mut reduce_emoji_index = use_state(0 as usize);

    let reduce_emoji_listener = {
        set_state!([reduce_emoji, reduce_emoji_index], {
            if *reduce_emoji_index == EMOTICONS.len() {
                *reduce_emoji_index = 0;
            }
            *reduce_emoji = EMOTICONS[*reduce_emoji_index];
            *reduce_emoji_index += 1;
        })
    };
    // add this to gx
    return gxi! {
        Body [
           h1 ( const inner_html = h1_value, const on_click = reduce_emoji_listener.clone() ),
           div ( const on_click = reduce_emoji_listener ) [
            span ( inner_html = "reducer emoji :"),
            span ( inner_html = &reduce_emoji_index.to_string()[..] )
           ],
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
                button ( on_click = set_state!( counter, *counter += 1), inner_html = "+" ),
                button ( on_click = set_state!( counter, *counter -= 1), inner_html = &reduce_emoji.to_string()[..] )
            ]
        ]
    };
}
