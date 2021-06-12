use crate::*;

gxi! {
    pub Centre {}
    render {
        Pure [
            View ( set_vexpand = true ),
            View [
                View ( set_hexpand = true ),
                View [
                    #children
                ],
                View ( set_hexpand = true )
            ],
            View ( set_vexpand = true )
        ]
    }
}
