use gxi::{gxi, set_state, State, StrongNodeType, Text};

pub fn cat_fact() -> StrongNodeType {
    let cat_fact = State::new(String::new());

    let fetch_cat_fact = set_state!(
        async |_| {
            let resp = reqwest::get("https://catfact.ninja/fact?max_length=140")
                .await
                .unwrap();
            *(*cat_fact).borrow_mut() = resp.text().await.unwrap();
        },
        [cat_fact]
    );

    return gxi! {
        div [
            button ( const on_click = fetch_cat_fact ) [
                Text ( value = "fetch cat memes!" )
            ],
            p [
                Text ( value = &cat_fact[..])
            ],
        ]
    };
}
