use crate::*;
use serde::{Deserialize, Serialize};

enum Msg {
    Fetch(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatFactResp {
    pub length: u32,
    pub fact: String,
}

gxi! {
    CatFact {
        cat_fact : Option<CatFactResp> = None
    }
    render {
        Init ( on_init = || Msg::Fetch(true) ) [
            Button ( class = "btn btn-dark" , on_click = || Msg::Fetch(false), inner_html = "Fetch Cat Memes" ),
            Div [
                if state.cat_fact.is_none()
                    Div ( class = "spinner-border text-info" )
                else
                    H3 ( class = "text-light", inner_html = &state.cat_fact.as_ref().unwrap().fact )
            ]
        ]
    }
    update async {
        match msg {
            Msg::Fetch(force) => {
                if {
                    let mut state = get_mut_state!(state);
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
                    let cat_fact = resp.json::<CatFactResp>().await?;
                    let mut state = get_mut_state!(state);
                    state.cat_fact = Some(cat_fact);
                    Ok(ShouldRender::Yes)
                } else {
                    Ok(ShouldRender::No)
                }
            }
        }
    }
}