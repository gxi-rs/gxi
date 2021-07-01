use std::collections::HashSet;

use crate::*;

const ENTER_KEY_CODE: u32 = 13;

enum Msg {
    Input(web_sys::KeyboardEvent),
    RmItem(Rc<String>),
}

gxi! {
    pub Todo {
        todo_map : HashSet<Rc<String>>,
        input_field : Option<WeakNodeType>
    }
    render {
        Div [
            H1 ( inner_html = "Todo app" ),
            Div [
                Input ( ref = state.input_field, on_keyup = |e| Msg::Input(e), id = "todo_input", value = "asd")
            ],
            Div ( title = "todo" ) [
                for todo in &state.todo_map where todo:Rc<String> {
                    Div ( class = "todo_task" ) [
                        {
                            let todo_clone = todo.clone();
                            log!("todo {} ", &todo_clone);
                        },
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
                    /*let mut state = get_state_mut!(state);
                    let input_field = unwrap_node!(state.input_field as Input);
                    let value = input_field.get_attribute("value").unwrap();
                    let len = state.todo_map.len();
                    log!("adding todo {} ", value);
                    state.todo_map.insert(Rc::new(format!("{}{}",value, len)));
                    return Ok(ShouldRender::Yes)*/
                }
            },
            Msg::RmItem(i) => {
                let mut state = get_state_mut!(state);
                log!("removing todo {} ", &i);
                state.todo_map.remove(&i);
                return Ok(ShouldRender::Yes);
            }
        }
        Ok(ShouldRender::No)
    }
}
