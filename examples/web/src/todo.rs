use gxi::{gxi, set_state, State, Text, VNodeContext};

pub fn todo() -> VNodeContext<gxi::Element> {
    let todos = State::from(String::new());
    let state2 = State::from(String::new());

    gxi! {
        div [
            input ( on_input = set_state!(|e| {
                let data = e.data().unwrap_or_default();
                todos.push_str(&data);
                state2.push_str(&data);
            }, [ref todos,ref state2]) ),
            div [
                Text ( value = "3rd element" )
            ],
            Text ( value = &*todos ),
            if *todos == "abc" {
                {
                    panic!("abc");
                },
                Text ( value = "abcd" ),
                Text ( value = "_foo")
            },
            div [
                Text ( value = "4th element" )
            ],
            if *todos == "abcdefgh" {
                Text ( value = "*_*")
            },
            if *todos == "abcdefghi" {
                Text ( value = "yay" )
            },
            div [
                Text ( value = "5th element" )
            ]
        ]
    }
}
