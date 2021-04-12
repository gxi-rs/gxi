use rust_gui::*;

comp! {
    Centre {
         cat_fact : Option<String> = None
    }
    render {
        Pure [
            View ( v_expand = true ),
            View [
                View ( h_expand = true ),
                #children
                View ( h_expand = true )
            ],
            View ( v_expand = true )
        ]
    }
}