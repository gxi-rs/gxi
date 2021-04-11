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
/*
let cont = node.clone();
let mut this_borrow = this.as_ref().borrow_mut();
let this =  this_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
this.parent_substitute = Rc::downgrade(&cont);
*/