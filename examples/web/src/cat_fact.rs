use gxi::{gxi, mod_state , AsyncState, State, StrongNodeType};

#[gxi::comp]
pub unsafe fn CatFact() -> StrongNodeType {
    let cat_fact = AsyncState::new(String::new());

    ////// dammnnn this is not allowed, let me play cs, ill fix this later
    #[mod_state(cat_fact)]
    let fetch_cat_fact = move |_| {
        let resp = reqwest::get("https://catfact.ninja/fact?max_length=140")
            .await
            .unwrap();
        *cat_fact.lock().unwrap() = resp.text().await.unwrap();
    };

    return gxi! {
        div [
            button ( const on_click = fetch_cat_fact , inner_html = "fetch cat memes"),
            p ( inner_html = &cat_fact[..] )
        ]
    };
}
