use gxi::{gxi, set_state, State, Text, VNodeContext};

const EMOTICONS: [&str; 3] = ["-", "🙃", "|"];

pub fn complex_counter() -> VNodeContext<gxi::Element> {
    let reduce_emoji = State::from(EMOTICONS[0]);
    let reduce_emoji_index = State::from(0usize);

    let reduce_emoji_listener = set_state! {|_| {
        if *reduce_emoji_index == EMOTICONS.len() {
            *reduce_emoji_index = 0;
        }
        *reduce_emoji = EMOTICONS[*reduce_emoji_index];
        *reduce_emoji_index += 1;
    }, [ref reduce_emoji, ref reduce_emoji_index]};

    // add this to gx
    gxi! {
        div [
           h1 ( const on_click = reduce_emoji_listener.clone() ) [
               Text ( value = "hello")
           ],
           div ( const on_click = reduce_emoji_listener ) [
               Text ( value = "reducer emoji :"),
               Text ( value = &reduce_emoji_index.to_string()[..] )
           ],
           counter(2, reduce_emoji.clone()),
           counter(20, reduce_emoji)
        ]
    }
}

fn counter(initial: i32, reduce_emoji: State<&'static str>) -> VNodeContext<gxi::Element> {
    let counter = State::from(initial);

    gxi! {
        div [
            h1 [
                Text ( value = counter.to_string() )
            ],
            div [
                button ( on_click = set_state!(*counter += 1, [ref counter])) [
                    Text( value = "+")
                ],
                button ( on_click = set_state!(*counter -= 1, [ref counter])) [
                    Text( value = &reduce_emoji.to_string()[..] )
                ]
            ]
        ]
    }
}
