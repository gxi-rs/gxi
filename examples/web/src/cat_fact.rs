use serde_json::Value;

use crate::*;

enum Msg {
    Fetch(bool),
}

gxi! {
    pub async CatFact {
        cat_fact : Option<String> = None
    }
    render {
        Init ( on_init = || Msg::Fetch(true) ) [
            Button ( class = "btn btn-dark" , on_click = |_| Msg::Fetch(false), inner_html = "Fetch Cat Memes" ),
            Div [
                if state.cat_fact.is_none() {
                    Div ( class = "spinner-border text-info" )
                } else {
                    H3 ( class = "text-light", inner_html = &state.cat_fact.as_ref().unwrap() )
                }
            ],
            Todo
        ]
    }
    update {
        match msg {
            Msg::Fetch(force) => {
                if {
                    let mut state = get_state_mut!(state);
                    if state.cat_fact.is_some() {
                        state.cat_fact = None;
                        drop(state);
                        render();
                        true
                    } else {
                        false
                    }
                } || force
                {
                    let resp = reqwest::get("https://catfact.ninja/fact?max_length=140").await?;
                    let cat_fact:Value = serde_json::from_str(&resp.text().await?)?;
                    let mut state = get_state_mut!(state);
                    state.cat_fact = Some(cat_fact["fact"].to_string());
                    Ok(ShouldRender::Yes)
                } else {
                    Ok(ShouldRender::No)
                }
            }
        }
    }
}
