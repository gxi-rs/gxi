use rust_gui::*;

comp! {
    Centre {}
    render {
        Pure [
            View ( v_expand = true ),
            View [
                View ( h_expand = true ),
                View [
                    #children
                ],
                View ( h_expand = true )
            ],
            View ( v_expand = true )
        ]
    }
}
