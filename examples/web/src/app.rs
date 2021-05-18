use crate::*;

gxi! {
    App {}
    render {
        Div [
            Body ( style = r#"background-color : #121212;"# ),
            Head [
                Title ( inner_html = "Hello World" ),
                Link ( rel = "stylesheet", href = "https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" ),
                Meta ( name = "viewport", content = "width=device-width, initial-scale=1" ),
                Script ( r#async = true )
            ],
            A ( href = "https://webbuddy360.com" ) [
                H1 ( inner_html = "hello world" ),
            ],
            Counter,
            CatFact,
            H1 ( inner_html = "Hi i am refreshed" ),
            H1 ( inner_html = "Hi i am refreshed" )
        ]
    }
}
