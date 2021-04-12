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
/*
let cont = node.clone();
let mut this_borrow = this.as_ref().borrow_mut();
let this =  this_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
this.self_substitute = Rc::downgrade(&cont);
*/