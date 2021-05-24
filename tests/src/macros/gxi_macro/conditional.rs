use gxi::*;

gxi! {
    App {
        limit:u32 = 2;
    }
    render {
        Comp [
            { /* First comment */ }
            if true {
                Comp ( id = "if1" , class = "if" ) [

                ]
                { println!("true is true") }
            }
            {/* Comment after if block */}
            for x in state.limit {
                Comp {

                }
            }
            {/* Last Comment */}
        ]
    }
}

#[test]
fn traverse_conditional() {
    println!("traversing")
}