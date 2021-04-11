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
                Image ( source = "cat.gif" ),
                View ( h_expand = true )
            ],
            View ( v_expand = true )
        ]
    }
}