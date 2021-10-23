use gxi::{gxi, set_state, State, StrongNodeType};

#[gxi::comp]
pub unsafe fn CatFact() -> StrongNodeType {
    let cat_fact = State::new(String::new());

    ////// dammnnn this is not allowed, let me play cs, ill fix this later
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
            button ( const on_click = fetch_cat_fact , inner_html = "fetch cat memes"),
            p ( inner_html = &cat_fact[..] )
        ]
    };
}
