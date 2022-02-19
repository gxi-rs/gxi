use gxi::{gxi, set_state, State, Text, VNodeContext};

pub fn cat_fact() -> VNodeContext<gxi::Element> {
    let cat_fact = State::from(String::new());

    let fetch_cat_fact = set_state!(
        async || {
            let resp = reqwest::get("https://catfact.ninja/fact?max_length=140")
                .await
                .unwrap();
            *(*cat_fact).borrow_mut() = resp.text().await.unwrap();
        },
        [cat_fact]
    );

    fetch_cat_fact();

    gxi! {
        div [
            button ( on_click = move |_| fetch_cat_fact() ) [
                Text ( value = "fetch cat memes!" )
            ],
            p [
                Text ( value = &cat_fact[..])
            ],
        ]
    }
}
