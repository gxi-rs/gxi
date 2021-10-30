use gxi::{gxi, State, StrongNodeType, Text, set_state};

pub fn todo() -> StrongNodeType {
    let todos = State::new(String::new());
   
    return gxi! {
        div [
            input ( on_input = set_state!(|e| {
               let data = e.data().unwrap_or(String::new());
               todos.push_str(&data);
            }, [ref todos] ) ),
            for x in todos {
                    span [
                        Text ( value = &todos[..] )
                    ]
                }
            }
        ]
    };
}
