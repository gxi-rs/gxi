use quote::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};
use syn::*;

use crate::TreeParser;

pub struct CompParser {
    pub tree: TokenStream2,
}

#[macro_export]
macro_rules! comp_init {
    ($name:ident $state_name:ident { $($p:ident : $t:ty = $v:expr)* } { $($render:tt)* } )=> {
        use std::any::Any;
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Mutex, Arc};
        #[cfg(feature = "desktop")]
        use glib::Sender;

        type AsyncState = Arc<Mutex<$state_name>>;

        pub struct $name {
            pub state: AsyncState,
            #[cfg(feature = "desktop")]
            pub channel_sender: Sender<()>,
            pub parent: WeakNodeRc,
            pub self_substitute : Option<WeakNodeRc>,
            pub dirty: bool,
            pub child: Option<NodeRc>,
            pub sibling: Option<NodeRc>
        }

        pub struct $state_name {
            $(pub $p:$t),*
        }

        impl Node for $name {
            impl_node_for_component!();

            fn new(parent: WeakNodeRc) -> NodeRc {
                #[cfg(feature = "desktop")]
                let (channel_sender, re) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
                let this:NodeRc = Rc::new(RefCell::new(Box::new(Self {
                    state: Arc::new(Mutex::new($state_name {
                        $($p:$v),*
                    })),
                    #[cfg(feature = "desktop")]
                    channel_sender,
                    self_substitute : None,
                    parent,
                    dirty: true,
                    child: None,
                    sibling: None,
                })));
                {
                    let mut this_borrow = this.as_ref().borrow_mut();
                    let this_borrow = this_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                    this_borrow.self_substitute = Some(Rc::downgrade(&this));
                }
                #[cfg(feature = "desktop")]
                {
                    let this = this.clone();
                    re.attach(None, move |_| {
                        let this = Rc::clone(&this);
                        //mark dirty
                        {
                            let mut node = this.as_ref().borrow_mut();
                            node.mark_dirty();
                        }
                        Self::render(this);
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
        let state_block = input.parse::<syn::Block>()?;
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
                                    let mut node_borrow = this.as_ref().borrow_mut();
                                    let node = node_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                                    if !node.is_dirty() {
                                        return;
                                    }
                                    node.mark_clean();
                                    node.state.clone()
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
            tree: quote!(comp_init!(#name #state_name #state_block {#render_func});),
        })
    }
}
