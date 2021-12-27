use gxi::{gxi, set_state, State, StrongNodeType, Text};

pub fn todo() -> StrongNodeType {
    let todos = State::new(String::new());

    let state2 = State::new(String::new());
    let state3 = State::new(String::new());

    return gxi! {
        div [
            input ( on_input = set_state!(|e| {
                let data = e.data().unwrap_or(String::new());
                todos.push_str(&data);
                state2.push_str(&data);
            }, [ref todos,ref state2]) ),
            if *todos == "a" {
                div [ Text ( value = "hi" ) ]
            } else if const *todos == "ab" {
                div [ Text ( value = "hi brother" ) ]
            } else if const *todos == "abcd"{
                Text ( value = "z" )
            } else {
                div [
                    Text ( value = "none" ),
                    if *state2 == "abcde" || *state2 == "abcdefg" {
                        div [
                            Text ( value = "zzzzzzzzzz" )
                        ]
                    }
                ]
            },
            div [
                Text ( value = "3rd element" )
            ],
            if *todos == "abc" {
                Text ( value = "abcd" ),
                Text ( value = "_foo") 
            },
            div [
                Text ( value = "4th element" )
            ],
            if *todos == "abcdefgh" {
//                if *state3.borrow() == "abcdefgh" {
                    Text ( value = "*_*")
//                }
            },
            if *todos == "abcdefghi" {
                Text ( value = "yay" )
            },
            div [
                Text ( value = "5th element" )
            ],
        ]
    };
}
