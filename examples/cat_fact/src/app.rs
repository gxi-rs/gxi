use serde::{Deserialize, Serialize};

use rust_gui::Orientation::Vertical;
use rust_gui::*;

use crate::centre::Centre;
use crate::counter::Counter;

enum Msg {
    Fetch(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatFact {
    pub length: u32,
    pub fact: String,
}

comp! {
    App {
        cat_fact : Option<CatFact> = None
    }
    render {
        Init ( on_init = || Msg::Fetch(true) ) [
            View ( orientation = Vertical ) [
                Centre [
                    Image ( source = "cat.gif" )
                ],
                Button ( on_click = || Msg::Fetch(false), label = "Fetch Cat Memes" ),
                View [
                    if state.cat_fact.is_none()
                        Pure [
                            Text ( label = "loading" ),
                            Spinner ( spin = true )
                        ]
                    else
                        Text ( label = &state.cat_fact.as_ref().unwrap().fact )
                ],
                Counter ( count = if let Some(cat_fact) = &state.cat_fact { Some(cat_fact.length) } else { None } )
            ]
        ]
    }
}

#[update(App)]
async fn update<F: Fn() + 'static>(
    state: AsyncState, msg: Msg, render: F,
) -> AsyncResult<ShouldRender> {
    match msg {
        Msg::Fetch(force) => {
            if {
                let mut state = state.lock().unwrap();
                if state.cat_fact.is_some() {
                    state.cat_fact = None;
                    render();
                    true
                } else {
                    false
                }
            } || force
            {
                let resp = reqwest::get("https://catfact.ninja/fact?max_length=140").await?;
                let cat_fact = resp.json::<CatFact>().await?;
                let mut state = state.lock().unwrap();
                state.cat_fact = Some(cat_fact);
                Ok(ShouldRender::Yes)
            } else {
                Ok(ShouldRender::No)
            }
        }
    }
}
