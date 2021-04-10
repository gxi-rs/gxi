use quote::*;
use syn::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};

use crate::TreeParser;

pub struct CompParser {
    pub tree: TokenStream2,
}

#[macro_export]
macro_rules! comp_init {
    ($name:ident $state_name:ident { $($p:ident : $t:ty = $v:expr);* } { $($render:tt)* } )=> {
        use std::any::Any;
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Mutex, Arc};
        use crate::glib::Sender;

        type AsyncState = Arc<Mutex<$state_name>>;

        pub struct $name {
            pub state: AsyncState,
            pub channel_sender: Sender<()>,
            pub parent: WeakNodeRc,
            pub dirty: bool,
            pub child: Option<NodeRc>,
            pub sibling: Option<NodeRc>,
            pub widget: gtk::Container,
        }

        pub struct $state_name {
            pub $($p:$t),*
        }

        impl Node for $name {
            impl_node_for_component!();

            fn new(parent: WeakNodeRc) -> NodeRc {
                let (channel_sender, re) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
                let this:NodeRc = Rc::new(RefCell::new(Box::new(Self {
                    state: Arc::new(Mutex::new($state_name {
                        $($p:$v),*
                    })),
                    channel_sender,
                    widget: parent.upgrade().unwrap().as_ref().borrow().get_widget_as_container(),
                    parent,
                    dirty: true,
                    child: None,
                    sibling: None,
                })));
                {
                    let this = this.clone();
                    re.attach(None, move |_| {
                        Self::render(Rc::clone(&this));
                        glib::Continue(true)
                    });
                }
                this
            }
            $($render)*
        }

        impl_drop_for_component!($name);
    };
}

impl Parse for CompParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let state_name = syn::Ident::new(&format!("{}State", quote! {#name}), Span::call_site());
        let props = input.parse::<syn::Block>()?;
        let mut render_func = quote!(
            fn render(_this: NodeRc) {}
        );
        for _ in 0..2 {
            if let Ok(s) = input.parse::<syn::Ident>() {
                match &s.to_string()[..] {
                    "render" => {
                        let block_content = group::parse_braces(&input)?.content;
                        let content = TreeParser::parse(&block_content)?.tree;
                        render_func = quote!(
                            fn render(this: NodeRc) {
                                let cont = Rc::clone(&this);
                                let node = cont.clone();
                                let state = {
                                    let state_borrow = this.as_ref().borrow();
                                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                                    state.state.clone()
                                };
                                let state = state.lock().unwrap();
                                #content
                            }
                        );
                    }
                    _ => panic!("Didn't expect this attribute here"),
                }
            }
        }
        Ok(CompParser {
            tree: quote!(comp_init!(#name #state_name #props {#render_func});),
        })
    }
}
