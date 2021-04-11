use rust_gui::*;

//TODO: Pure not working as node here
comp! {
    Centre {
         cat_fact : Option<String> = None
    }
    render {
        View [
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