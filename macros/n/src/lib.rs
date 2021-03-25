/*!
[n!](macro.n.html) macro expands given widget to correct method calls.

It is adviced not to use it directly, use [c!](../c/macro.c.html) macro instead.
*Example*
```
 use std::cell::RefCell;
 use std::rc::Rc;
 use n::n;
 use gtk::{ButtonExt, WidgetExt};

 fn render(container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
     //these 2 lines are mandatory
     let cont = Rc::clone(&container);
     let node = cont.clone();
     //init_child for first child, init_sibling for other children
     n!(View init_child { set_property_width_request = 300 ; });
     {//make sure to use brackets in order to separate hierarchy
         let cont = node.clone(); //this is important
         n!(Button init_child { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; });
         n!(Button init_sibling { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; });
         n!(View init_sibling {});
         {
             let cont = node.clone();
             n!(Button init_child { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; });
         }
     }
 }
```
!*/

use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;

use crate::combinations::Combinations;

mod combinations;

#[proc_macro]
pub fn n(item: TokenStream) -> TokenStream {
    let Combinations { name, static_exprs, dynamic_exprs, init_type, pure_index } = syn::parse_macro_input!(item as Combinations);

    let (pure_state_reference, pure_remove_block) = if pure_index > 0 {
        (TokenStream2::new(), quote! {
            let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
            if pure.current_index != #pure_index {
                if pure.child.is_some() {
                    let child = pure.child.as_ref().unwrap();
                    pure.get_widget_as_container().remove(&child.as_ref().borrow().get_widget());
                    pure.child = None;
                }
                pure.current_index = #pure_index;
            }
         })
    } else {
        (quote! {
            let mut state_borrow = top_state.as_ref().borrow();
            let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
        }, TokenStream2::new())
    };

    (quote! {
        let node = {
            let (node, is_new) = {
                let widget = Some(cont.as_ref().borrow().get_widget_as_container());
                let mut node_borrow = node.as_ref().borrow_mut();
                { #pure_remove_block }
                let cont = Rc::clone(&cont);
                node_borrow.#init_type(Box::new(move || #name::new(cont.clone(),widget)), if let NodeType::Widget = #name::get_type() { true } else { false })
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<#name>().unwrap();
                if is_new {
                    #(#static_exprs)*
                }
                #pure_state_reference
                #(#dynamic_exprs)*
            }
            #name::render(node.clone());
            node
        };
    }).into()
}
