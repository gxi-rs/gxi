use std::collections::HashSet;

use crate::*;

const ENTER_KEY_CODE: u32 = 13;

enum Msg {
    Input(web_sys::KeyboardEvent),
    RmItem(String),
}

gxi! {
    pub Todo {
        todo_map : HashSet<String>
    }
    render {
        Div [
            H1 ( inner_html = "Todo app" ),
            Div [
                Input ( on_keyup = |e| Msg::Input(e), id = "todo_input", value = "asd")
            ],
            Div ( title = "todo" ) [
                for todo in &state.todo_map {
                    Div ( class = "todo_task" ) [
                        { let todo_clone = todo.clone(); },
                        Button ( inner_html = "X", on_click = |_| Msg::RmItem(todo_clone.clone()) ),
                        P ( inner_html = &todo.clone() )
                    ]
                }
            ],

        ]
    }
    update {
        match msg {
            Msg::Input(e) => {
                if e.key_code() == ENTER_KEY_CODE {
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let todo_input = document.get_element_by_id("todo_input").unwrap();
                    let value = todo_input.get_attribute("value").unwrap();
                    let mut state = get_state_mut!(state);
                    log!("adding todo {} ", value);
                    state.todo_map.insert(value);
                    return Ok(ShouldRender::Yes)
                }
            },
            Msg::RmItem(i) => {
                let mut state = get_state_mut!(state);
                state.todo_map.remove(&i);
                return Ok(ShouldRender::Yes);
            }
        }
        Ok(ShouldRender::No)
    }
}
