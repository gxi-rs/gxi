use crate::*;

gxi! {
    pub App {
    }
    render {
        Html ( lang = "en-us") [
            Head [
                Title ( inner_html = "Hello World" ),
                Link ( rel = "stylesheet", href = "https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" ),
                Link ( rel = "stylesheet", href = "/index.css" ),
                Meta ( name = "viewport", content = "width=device-width, initial-scale=1" ),
                Script ( r#async = true )
            ],
            Body ( style = r#"background-color : #121212;"# ) [
                Div [
                    A ( href = "https://webbuddy360.com" ) [
                        H1 ( inner_html = "hello world" ),
                    ],
                    Counter,
                    CatFact
                ]
            ]
        ]
    }
}
