use std::process::exit;

use serde::{Deserialize, Serialize};

use crate::*;

enum Msg {
    Fetch(bool),
    ShowHelp(bool),
    Quit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatFact {
    pub length: u32,
    pub fact: String,
}

gxi! {
    pub async App {
        cat_fact : Option<CatFact> = None,
        show_help : bool = false
    }
    render {
        Window ( connect_destroy = |_| Msg::Quit ) [
            Init ( on_init = || Msg::Fetch(true) ) [
                View ( set_orientation = gtk::Orientation::Vertical ) [
                    View [
                        if state.show_help {
                            Window ( connect_destroy = |_| Msg::ShowHelp(false) ) [
                                View ( set_orientation = gtk::Orientation::Vertical ) [
                                    Text ( set_label = "Cat Meme Fetcher By Aniket Prajapati made using gxi-rs."),
                                    Button ( set_label = "Ok take me back now", connect_clicked = |_| Msg::ShowHelp(false) )
                                ]
                            ],
                        } else {
                            Button ( set_label = "Show help", connect_clicked = |_| Msg::ShowHelp(true) ),
                        },
                    ],
                    Centre [
                        Image ( source = "cat.gif" )
                    ],
                    Button ( connect_clicked = |_| Msg::Fetch(false), set_label = "Fetch Cat Memes" ),
                    View [
                        if state.cat_fact.is_none() {
                            Text ( set_label = "loading" ),
                            Spinner ( spin = true )
                        } else {
                            Text ( set_label = "loaded" ),
                            Text ( set_label = &state.cat_fact.as_ref().unwrap().fact )
                        }
                    ],
                    Counter ( count = if let Some(cat_fact) = &state.cat_fact { Some(cat_fact.length) } else { None } ),
                ]
            ]
        ]
    }
    update {
        match msg {
            Msg::Fetch(force) => {
                if {
                    let mut state = get_state!(state);
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
                    let mut state = get_state!(state);
                    state.cat_fact = Some(cat_fact);
                    Ok(ShouldRender::Yes)
                } else {
                    Ok(ShouldRender::No)
                }
            },
            Msg::ShowHelp(should_show) => {
                get_state!(state).show_help = should_show;
                Ok(ShouldRender::Yes)
            },
            Msg::Quit => {
                exit(0)
            }
        }
    }
}
