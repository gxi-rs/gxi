use serde::{Deserialize, Serialize};

use rust_gui::*;

use crate::counter::Counter;

enum Msg {
    Fetch
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
        Init ( on_init = || Msg::Fetch ) [
            View [
                Image ( source = "cat.gif" ),
                Button ( on_click = || Msg::Fetch, label = "Fetch Cat Memes" ),
                View [
                    if state.cat_fact.is_none()
                        Spinner ( spin = true )
                    else
                        Text ( label = &state.cat_fact.as_ref().unwrap().fact)
                ],
                Counter
            ]
        ]
    }
    update {
        match msg {
            Msg::Fetch => {
                //Todo! call render()
                let resp = reqwest::get("https://catfact.ninja/fact?max_length=140").await?;
                let json = resp.json::<CatFact>().await?;
                let mut state = state.lock().unwrap();
                state.cat_fact = Some(json);
            }
        }
        Ok(ShouldRender::Yes)
    }
}
