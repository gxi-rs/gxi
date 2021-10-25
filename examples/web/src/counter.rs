use gxi::{gxi, set_state, State, StrongNodeType};

const EMOTICONS: [&'static str; 3] = ["-", "🙃", "|"];

pub unsafe fn complex_counter() -> StrongNodeType {
    let h1_value = "hello";
    let reduce_emoji = State::new(EMOTICONS[0]);
    let reduce_emoji_index = State::new(0 as usize);

    let reduce_emoji_listener = set_state! {|_| {
        if *reduce_emoji_index == EMOTICONS.len() {
            *reduce_emoji_index = 0;
        }
        *reduce_emoji = EMOTICONS[*reduce_emoji_index];
        *reduce_emoji_index += 1;
    }, [ref reduce_emoji, ref reduce_emoji_index]};

    // add this to gx
    return gxi! {
        //TODO: pure component
        div [
           h1 ( const inner_html = h1_value, const on_click = reduce_emoji_listener.clone() ),
           div ( const on_click = reduce_emoji_listener ) [
            span ( inner_html = "reducer emoji :"),
            span ( inner_html = &reduce_emoji_index.to_string()[..] )
           ],
           counter(2, reduce_emoji.clone()),
           counter(20, reduce_emoji)
        ]
    };
}

unsafe fn counter(initial: i32, reduce_emoji: State<&'static str>) -> StrongNodeType {
    let counter = State::new(initial);

    return gxi! {
        div [
            h1 ( inner_html = &counter.to_string()[..] ),
            div [
                button ( on_click = set_state!(*counter += 1, [ref counter]), inner_html = "+" ),
                button ( on_click = set_state!(*counter -= 1, [ref counter]), inner_html = &reduce_emoji.to_string()[..] )
            ]
        ]
    };
}
