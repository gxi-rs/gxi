extern crate std;
pub use app::*;
pub use centre::*;
pub use counter::*;
pub use gxi::*;
mod app {
    use std::process::exit;
    use serde::{Deserialize, Serialize};
    use crate::*;
    enum Msg {
        Fetch(bool),
        ShowHelp(bool),
        Quit,
    }
    pub struct CatFact {
        pub length: u32,
        pub fact: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for CatFact {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                CatFact {
                    length: ref __self_0_0,
                    fact: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "CatFact");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "length",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "fact",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CatFact {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "CatFact",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "length",
                    &self.length,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "fact",
                    &self.fact,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CatFact {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            "length" => _serde::__private::Ok(__Field::__field0),
                            "fact" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            b"length" => _serde::__private::Ok(__Field::__field0),
                            b"fact" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CatFact>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CatFact;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct CatFact")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct CatFact with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct CatFact with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(CatFact {
                            length: __field0,
                            fact: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "length",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "fact",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("length") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("fact") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(CatFact {
                            length: __field0,
                            fact: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["length", "fact"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CatFact",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CatFact>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    use gxi::get_arc_state as get_state;
    use gxi::get_arc_state as get_state_mut;
    use std::any::Any;
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Mutex, Arc};
    use std::ops::DerefMut;
    type State = Arc<Mutex<AppState>>;
    pub struct AppState {
        cat_fact: Option<CatFact>,
        show_help: bool,
    }
    pub struct App {
        state: State,
        pub channel_sender: glib::Sender<()>,
        pub parent: WeakNodeType,
        pub self_substitute: Option<WeakNodeType>,
        pub is_dirty: bool,
        pub child: Option<StrongNodeType>,
        pub sibling: Option<StrongNodeType>,
    }
    impl Node for App {
        fn as_node(&self) -> &dyn Node {
            self
        }
        fn as_node_mut(&mut self) -> &mut dyn Node {
            self
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn get_parent(&self) -> &WeakNodeType {
            &self.parent
        }
        fn get_sibling(&self) -> &Option<StrongNodeType> {
            &self.sibling
        }
        fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.sibling
        }
        fn new(parent: WeakNodeType) -> StrongNodeType {
            let (channel_sender, re) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
                state: Arc::new(Mutex::new(AppState {
                    cat_fact: None,
                    show_help: false,
                })),
                channel_sender,
                self_substitute: None,
                parent,
                is_dirty: true,
                child: None,
                sibling: None,
            }))));
            {
                let this = this.clone();
                re.attach(None, move |_| {
                    let this = Rc::clone(&this);
                    {
                        let mut this_borrow = this.as_ref().borrow_mut();
                        match this_borrow.deref_mut() {
                            GxiNodeType::Component(t) => t.mark_dirty(),
                            _ => {
                                panic!("internal error: entered unreachable code")
                            }
                        }
                    }
                    Self::render(this);
                    glib::Continue(true)
                });
            }
            this
        }
        fn render(this: StrongNodeType) {
            let cont = Rc::clone(&this);
            let state = {
                let mut node = this.as_ref().borrow_mut();
                let node = node
                    .as_node_mut()
                    .as_any_mut()
                    .downcast_mut::<Self>()
                    .unwrap();
                if !node.is_dirty() {
                    return;
                }
                node.mark_clean();
                node.state.clone()
            };
            let state = state.lock().unwrap();
            let node = {
                let (node, is_new) =
                    init_member(cont.clone(), InitType::Child, |this| Init::new(this));
                {
                    let mut node = node.as_ref().borrow_mut();
                    let node = node
                        .as_node_mut()
                        .as_any_mut()
                        .downcast_mut::<Init>()
                        .unwrap();
                    if is_new {
                        {
                            let state_clone = Rc::clone(&this);
                            node.on_init(move || {
                                Self::update(state_clone.clone(), Msg::Fetch(true))
                            });
                        }
                    }
                }
                Init::render(node.clone());
                node
            };
            {
                let cont = {
                    let node_borrow = node.as_ref().borrow();
                    let node_borrow = node_borrow
                        .as_node()
                        .as_any()
                        .downcast_ref::<Init>()
                        .unwrap();
                    if let Some(subst) = node_borrow.get_self_substitute() {
                        subst.upgrade().unwrap()
                    } else {
                        node.clone()
                    }
                };
                let node = {
                    let (node, is_new) =
                        init_member(cont.clone(), InitType::Child, |this| View::new(this));
                    {
                        let mut node = node.as_ref().borrow_mut();
                        let node = node
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<View>()
                            .unwrap();
                        node.orientation(Orientation::Vertical);
                    }
                    View::render(node.clone());
                    node
                };
                {
                    let cont = {
                        let node_borrow = node.as_ref().borrow();
                        let node_borrow = node_borrow
                            .as_node()
                            .as_any()
                            .downcast_ref::<View>()
                            .unwrap();
                        if let Some(subst) = node_borrow.get_self_substitute() {
                            subst.upgrade().unwrap()
                        } else {
                            node.clone()
                        }
                    };
                    let node = {
                        let (node, is_new) =
                            init_member(cont.clone(), InitType::Child, |this| View::new(this));
                        View::render(node.clone());
                        node
                    };
                    {
                        let cont = {
                            let node_borrow = node.as_ref().borrow();
                            let node_borrow = node_borrow
                                .as_node()
                                .as_any()
                                .downcast_ref::<View>()
                                .unwrap();
                            if let Some(subst) = node_borrow.get_self_substitute() {
                                subst.upgrade().unwrap()
                            } else {
                                node.clone()
                            }
                        };
                        let (node, ..) =
                            init_member(node.clone(), InitType::Child, |this| Pure::new(this));
                        {
                            let cont = node.clone();
                            if state.show_help {
                                {
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let pure = node_borrow
                                        .as_node_mut()
                                        .as_any_mut()
                                        .downcast_mut::<Pure>()
                                        .unwrap();
                                    if pure.pure_index != 1u32 {
                                        pure.pure_index = 1u32;
                                        pure.child = None;
                                    }
                                }
                                let node = {
                                    let (node, is_new) =
                                        init_member(cont.clone(), InitType::Child, |this| {
                                            Window::new(this)
                                        });
                                    {
                                        let mut node = node.as_ref().borrow_mut();
                                        let node = node
                                            .as_node_mut()
                                            .as_any_mut()
                                            .downcast_mut::<Window>()
                                            .unwrap();
                                        if is_new {
                                            {
                                                let state_clone = Rc::clone(&this);
                                                node.on_destroy(move || {
                                                    Self::update(
                                                        state_clone.clone(),
                                                        Msg::ShowHelp(false),
                                                    )
                                                });
                                            }
                                        }
                                    }
                                    Window::render(node.clone());
                                    node
                                };
                                {
                                    let cont = {
                                        let node_borrow = node.as_ref().borrow();
                                        let node_borrow = node_borrow
                                            .as_node()
                                            .as_any()
                                            .downcast_ref::<Window>()
                                            .unwrap();
                                        if let Some(subst) = node_borrow.get_self_substitute() {
                                            subst.upgrade().unwrap()
                                        } else {
                                            node.clone()
                                        }
                                    };
                                    let node = {
                                        let (node, is_new) =
                                            init_member(cont.clone(), InitType::Child, |this| {
                                                View::new(this)
                                            });
                                        {
                                            let mut node = node.as_ref().borrow_mut();
                                            let node = node
                                                .as_node_mut()
                                                .as_any_mut()
                                                .downcast_mut::<View>()
                                                .unwrap();
                                            node.orientation(Orientation::Vertical);
                                        }
                                        View::render(node.clone());
                                        node
                                    };
                                    {
                                        let cont = {
                                            let node_borrow = node.as_ref().borrow();
                                            let node_borrow = node_borrow
                                                .as_node()
                                                .as_any()
                                                .downcast_ref::<View>()
                                                .unwrap();
                                            if let Some(subst) = node_borrow.get_self_substitute() {
                                                subst.upgrade().unwrap()
                                            } else {
                                                node.clone()
                                            }
                                        };
                                        let node = {
                                            let (node, is_new) = init_member(
                                                cont.clone(),
                                                InitType::Child,
                                                |this| Text::new(this),
                                            );
                                            {
                                                let mut node = node.as_ref().borrow_mut();
                                                let node = node
                                                    .as_node_mut()
                                                    .as_any_mut()
                                                    .downcast_mut::<Text>()
                                                    .unwrap();
                                                if is_new {
                                                    node . label ("Cat Meme Fetcher By Aniket Prajapati made using gxi-rs.") ;
                                                }
                                            }
                                            Text::render(node.clone());
                                            node
                                        };
                                        {}
                                        Text::render(node.clone());
                                        let node = {
                                            let (node, is_new) = init_member(
                                                node.clone(),
                                                InitType::Sibling,
                                                |this| Button::new(this),
                                            );
                                            {
                                                let mut node = node.as_ref().borrow_mut();
                                                let node = node
                                                    .as_node_mut()
                                                    .as_any_mut()
                                                    .downcast_mut::<Button>()
                                                    .unwrap();
                                                if is_new {
                                                    node.label("Ok take me back now");
                                                    {
                                                        let state_clone = Rc::clone(&this);
                                                        node.on_click(move || {
                                                            Self::update(
                                                                state_clone.clone(),
                                                                Msg::ShowHelp(false),
                                                            )
                                                        });
                                                    }
                                                }
                                            }
                                            Button::render(node.clone());
                                            node
                                        };
                                        {}
                                        Button::render(node.clone());
                                    }
                                    View::render(node.clone());
                                }
                                Window::render(node.clone());
                            } else {
                                {
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let pure = node_borrow
                                        .as_node_mut()
                                        .as_any_mut()
                                        .downcast_mut::<Pure>()
                                        .unwrap();
                                    if pure.pure_index != 2u32 {
                                        pure.pure_index = 2u32;
                                        pure.child = None;
                                    }
                                }
                                let node = {
                                    let (node, is_new) =
                                        init_member(cont.clone(), InitType::Child, |this| {
                                            Button::new(this)
                                        });
                                    {
                                        let mut node = node.as_ref().borrow_mut();
                                        let node = node
                                            .as_node_mut()
                                            .as_any_mut()
                                            .downcast_mut::<Button>()
                                            .unwrap();
                                        if is_new {
                                            node.label("Show help");
                                            {
                                                let state_clone = Rc::clone(&this);
                                                node.on_click(move || {
                                                    Self::update(
                                                        state_clone.clone(),
                                                        Msg::ShowHelp(true),
                                                    )
                                                });
                                            }
                                        }
                                    }
                                    Button::render(node.clone());
                                    node
                                };
                                {}
                                Button::render(node.clone());
                            }
                        }
                    }
                    View::render(node.clone());
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Sibling, |this| Button::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<Button>()
                                .unwrap();
                            if is_new {
                                {
                                    let state_clone = Rc::clone(&this);
                                    node.on_click(move || {
                                        Self::update(state_clone.clone(), Msg::Fetch(false))
                                    });
                                }
                                node.label("Fetch Cat Memes");
                            }
                        }
                        Button::render(node.clone());
                        node
                    };
                    {}
                    Button::render(node.clone());
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Sibling, |this| View::new(this));
                        View::render(node.clone());
                        node
                    };
                    {
                        let cont = {
                            let node_borrow = node.as_ref().borrow();
                            let node_borrow = node_borrow
                                .as_node()
                                .as_any()
                                .downcast_ref::<View>()
                                .unwrap();
                            if let Some(subst) = node_borrow.get_self_substitute() {
                                subst.upgrade().unwrap()
                            } else {
                                node.clone()
                            }
                        };
                        let (node, ..) =
                            init_member(node.clone(), InitType::Child, |this| Pure::new(this));
                        {
                            let cont = node.clone();
                            if state.cat_fact.is_none() {
                                {
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let pure = node_borrow
                                        .as_node_mut()
                                        .as_any_mut()
                                        .downcast_mut::<Pure>()
                                        .unwrap();
                                    if pure.pure_index != 1u32 {
                                        pure.pure_index = 1u32;
                                        pure.child = None;
                                    }
                                }
                                let node = {
                                    let (node, is_new) =
                                        init_member(cont.clone(), InitType::Child, |this| {
                                            Pure::new(this)
                                        });
                                    Pure::render(node.clone());
                                    node
                                };
                                {
                                    let cont = {
                                        let node_borrow = node.as_ref().borrow();
                                        let node_borrow = node_borrow
                                            .as_node()
                                            .as_any()
                                            .downcast_ref::<Pure>()
                                            .unwrap();
                                        if let Some(subst) = node_borrow.get_self_substitute() {
                                            subst.upgrade().unwrap()
                                        } else {
                                            node.clone()
                                        }
                                    };
                                    let node = {
                                        let (node, is_new) =
                                            init_member(cont.clone(), InitType::Child, |this| {
                                                Text::new(this)
                                            });
                                        {
                                            let mut node = node.as_ref().borrow_mut();
                                            let node = node
                                                .as_node_mut()
                                                .as_any_mut()
                                                .downcast_mut::<Text>()
                                                .unwrap();
                                            if is_new {
                                                node.label("loading");
                                            }
                                        }
                                        Text::render(node.clone());
                                        node
                                    };
                                    {}
                                    Text::render(node.clone());
                                    let node = {
                                        let (node, is_new) =
                                            init_member(node.clone(), InitType::Sibling, |this| {
                                                Spinner::new(this)
                                            });
                                        {
                                            let mut node = node.as_ref().borrow_mut();
                                            let node = node
                                                .as_node_mut()
                                                .as_any_mut()
                                                .downcast_mut::<Spinner>()
                                                .unwrap();
                                            if is_new {
                                                node.spin(true);
                                            }
                                        }
                                        Spinner::render(node.clone());
                                        node
                                    };
                                    {}
                                    Spinner::render(node.clone());
                                }
                                Pure::render(node.clone());
                            } else {
                                {
                                    let mut node_borrow = node.as_ref().borrow_mut();
                                    let pure = node_borrow
                                        .as_node_mut()
                                        .as_any_mut()
                                        .downcast_mut::<Pure>()
                                        .unwrap();
                                    if pure.pure_index != 2u32 {
                                        pure.pure_index = 2u32;
                                        pure.child = None;
                                    }
                                }
                                let node = {
                                    let (node, is_new) =
                                        init_member(cont.clone(), InitType::Child, |this| {
                                            Text::new(this)
                                        });
                                    {
                                        let mut node = node.as_ref().borrow_mut();
                                        let node = node
                                            .as_node_mut()
                                            .as_any_mut()
                                            .downcast_mut::<Text>()
                                            .unwrap();
                                        node.label(&state.cat_fact.as_ref().unwrap().fact);
                                    }
                                    Text::render(node.clone());
                                    node
                                };
                                {}
                                Text::render(node.clone());
                            }
                        }
                    }
                    View::render(node.clone());
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Sibling, |this| Counter::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<Counter>()
                                .unwrap();
                            node.count(if let Some(cat_fact) = &state.cat_fact {
                                Some(cat_fact.length)
                            } else {
                                None
                            });
                        }
                        Counter::render(node.clone());
                        node
                    };
                    {}
                    Counter::render(node.clone());
                }
                View::render(node.clone());
            }
            Init::render(node.clone());
        }
    }
    impl Container for App {
        fn get_child(&self) -> &Option<StrongNodeType> {
            &self.child
        }
        fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.child
        }
        fn as_container(&self) -> &dyn Container {
            self
        }
        fn as_container_mut(&mut self) -> &mut dyn Container {
            self
        }
    }
    impl ComponentNode for App {
        fn get_self_substitute(&self) -> &Option<WeakNodeType> {
            &self.self_substitute
        }
        fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
            &mut self.self_substitute
        }
        fn is_dirty(&self) -> bool {
            self.is_dirty
        }
        fn mark_dirty(&mut self) {
            self.is_dirty = true
        }
        fn mark_clean(&mut self) {
            self.is_dirty = false
        }
    }
    impl App {
        fn update(this: StrongNodeType, msg: Msg) {
            let (channel_sender, state) = {
                let state_borrow = this.as_ref().borrow();
                let state = state_borrow
                    .as_node()
                    .as_any()
                    .downcast_ref::<App>()
                    .unwrap();
                (state.channel_sender.clone(), state.state.clone())
            };
            tokio::task::spawn(async move {
                let render = {
                    let channel_sender = channel_sender.clone();
                    move || channel_sender.send(()).unwrap()
                };
                async fn update<F: Fn() + 'static>(
                    state: State,
                    msg: Msg,
                    render: F,
                ) -> AsyncResult<ShouldRender> {
                    match msg {
                        Msg::Fetch(force) => {
                            if {
                                let mut state = state.lock().unwrap();
                                if state.cat_fact.is_some() {
                                    state.cat_fact = None;
                                    render();
                                    true
                                } else {
                                    false
                                }
                            } || force
                            {
                                let resp =
                                    reqwest::get("https://catfact.ninja/fact?max_length=140")
                                        .await?;
                                let cat_fact = resp.json::<CatFact>().await?;
                                let mut state = state.lock().unwrap();
                                state.cat_fact = Some(cat_fact);
                                Ok(ShouldRender::Yes)
                            } else {
                                Ok(ShouldRender::No)
                            }
                        }
                        Msg::ShowHelp(should_show) => {
                            state.lock().unwrap().show_help = should_show;
                            Ok(ShouldRender::Yes)
                        }
                        Msg::Quit => exit(0),
                    }
                }
                if let ShouldRender::Yes = update(state, msg, render).await.unwrap() {
                    channel_sender.send(()).unwrap()
                }
            });
        }
    }
}
mod centre {
    use crate::*;
    use gxi::get_rc_state as get_state;
    use gxi::get_mut_rc_state as get_state_mut;
    use std::any::Any;
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Mutex, Arc};
    use std::ops::DerefMut;
    type State = Rc<RefCell<CentreState>>;
    pub struct CentreState {}
    pub struct Centre {
        state: State,
        pub parent: WeakNodeType,
        pub self_substitute: Option<WeakNodeType>,
        pub is_dirty: bool,
        pub child: Option<StrongNodeType>,
        pub sibling: Option<StrongNodeType>,
    }
    impl Node for Centre {
        fn as_node(&self) -> &dyn Node {
            self
        }
        fn as_node_mut(&mut self) -> &mut dyn Node {
            self
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn get_parent(&self) -> &WeakNodeType {
            &self.parent
        }
        fn get_sibling(&self) -> &Option<StrongNodeType> {
            &self.sibling
        }
        fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.sibling
        }
        fn new(parent: WeakNodeType) -> StrongNodeType {
            let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
                state: Rc::new(RefCell::new(CentreState {})),
                self_substitute: None,
                parent,
                is_dirty: true,
                child: None,
                sibling: None,
            }))));
            this
        }
        fn render(this: StrongNodeType) {
            let cont = Rc::clone(&this);
            let state = {
                let mut node = this.as_ref().borrow_mut();
                let node = node
                    .as_node_mut()
                    .as_any_mut()
                    .downcast_mut::<Self>()
                    .unwrap();
                if !node.is_dirty() {
                    return;
                }
                node.mark_clean();
                node.state.clone()
            };
            let state = state.as_ref().borrow();
            let node = {
                let (node, is_new) =
                    init_member(cont.clone(), InitType::Child, |this| Pure::new(this));
                Pure::render(node.clone());
                node
            };
            {
                let cont = {
                    let node_borrow = node.as_ref().borrow();
                    let node_borrow = node_borrow
                        .as_node()
                        .as_any()
                        .downcast_ref::<Pure>()
                        .unwrap();
                    if let Some(subst) = node_borrow.get_self_substitute() {
                        subst.upgrade().unwrap()
                    } else {
                        node.clone()
                    }
                };
                let node = {
                    let (node, is_new) =
                        init_member(cont.clone(), InitType::Child, |this| View::new(this));
                    {
                        let mut node = node.as_ref().borrow_mut();
                        let node = node
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<View>()
                            .unwrap();
                        if is_new {
                            node.v_expand(true);
                        }
                    }
                    View::render(node.clone());
                    node
                };
                {}
                View::render(node.clone());
                let node = {
                    let (node, is_new) =
                        init_member(node.clone(), InitType::Sibling, |this| View::new(this));
                    View::render(node.clone());
                    node
                };
                {
                    let cont = {
                        let node_borrow = node.as_ref().borrow();
                        let node_borrow = node_borrow
                            .as_node()
                            .as_any()
                            .downcast_ref::<View>()
                            .unwrap();
                        if let Some(subst) = node_borrow.get_self_substitute() {
                            subst.upgrade().unwrap()
                        } else {
                            node.clone()
                        }
                    };
                    let node = {
                        let (node, is_new) =
                            init_member(cont.clone(), InitType::Child, |this| View::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<View>()
                                .unwrap();
                            if is_new {
                                node.h_expand(true);
                            }
                        }
                        View::render(node.clone());
                        node
                    };
                    {}
                    View::render(node.clone());
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Sibling, |this| View::new(this));
                        View::render(node.clone());
                        node
                    };
                    {
                        let cont = {
                            let node_borrow = node.as_ref().borrow();
                            let node_borrow = node_borrow
                                .as_node()
                                .as_any()
                                .downcast_ref::<View>()
                                .unwrap();
                            if let Some(subst) = node_borrow.get_self_substitute() {
                                subst.upgrade().unwrap()
                            } else {
                                node.clone()
                            }
                        };
                        let node = {
                            let (node, ..) =
                                init_member(node.clone(), InitType::Child, |this| Pure::new(this));
                            {
                                let mut this_borrow = this.as_ref().borrow_mut();
                                match this_borrow.deref_mut() {
                                    GxiNodeType::Component(t) => {
                                        *t.get_self_substitute_mut() = Some(Rc::downgrade(&this))
                                    }
                                    _ => panic!(
                                        "internal error: entered unreachable code",
                                    ),
                                }
                            }
                            node
                        };
                    }
                    View::render(node.clone());
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Sibling, |this| View::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<View>()
                                .unwrap();
                            if is_new {
                                node.h_expand(true);
                            }
                        }
                        View::render(node.clone());
                        node
                    };
                    {}
                    View::render(node.clone());
                }
                View::render(node.clone());
                let node = {
                    let (node, is_new) =
                        init_member(node.clone(), InitType::Sibling, |this| View::new(this));
                    {
                        let mut node = node.as_ref().borrow_mut();
                        let node = node
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<View>()
                            .unwrap();
                        if is_new {
                            node.v_expand(true);
                        }
                    }
                    View::render(node.clone());
                    node
                };
                {}
                View::render(node.clone());
            }
            Pure::render(node.clone());
        }
    }
    impl Container for Centre {
        fn get_child(&self) -> &Option<StrongNodeType> {
            &self.child
        }
        fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.child
        }
        fn as_container(&self) -> &dyn Container {
            self
        }
        fn as_container_mut(&mut self) -> &mut dyn Container {
            self
        }
    }
    impl ComponentNode for Centre {
        fn get_self_substitute(&self) -> &Option<WeakNodeType> {
            &self.self_substitute
        }
        fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
            &mut self.self_substitute
        }
        fn is_dirty(&self) -> bool {
            self.is_dirty
        }
        fn mark_dirty(&mut self) {
            self.is_dirty = true
        }
        fn mark_clean(&mut self) {
            self.is_dirty = false
        }
    }
    impl Centre {}
}
mod counter {
    use crate::*;
    enum Msg {
        INC,
        DEC,
    }
    use gxi::get_rc_state as get_state;
    use gxi::get_mut_rc_state as get_state_mut;
    use std::any::Any;
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Mutex, Arc};
    use std::ops::DerefMut;
    type State = Rc<RefCell<CounterState>>;
    pub struct CounterState {
        count: u32,
    }
    pub struct Counter {
        state: State,
        pub parent: WeakNodeType,
        pub self_substitute: Option<WeakNodeType>,
        pub is_dirty: bool,
        pub child: Option<StrongNodeType>,
        pub sibling: Option<StrongNodeType>,
    }
    impl Node for Counter {
        fn as_node(&self) -> &dyn Node {
            self
        }
        fn as_node_mut(&mut self) -> &mut dyn Node {
            self
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn get_parent(&self) -> &WeakNodeType {
            &self.parent
        }
        fn get_sibling(&self) -> &Option<StrongNodeType> {
            &self.sibling
        }
        fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.sibling
        }
        fn new(parent: WeakNodeType) -> StrongNodeType {
            let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
                state: Rc::new(RefCell::new(CounterState { count: 0 })),
                self_substitute: None,
                parent,
                is_dirty: true,
                child: None,
                sibling: None,
            }))));
            this
        }
        fn render(this: StrongNodeType) {
            let cont = Rc::clone(&this);
            let state = {
                let mut node = this.as_ref().borrow_mut();
                let node = node
                    .as_node_mut()
                    .as_any_mut()
                    .downcast_mut::<Self>()
                    .unwrap();
                if !node.is_dirty() {
                    return;
                }
                node.mark_clean();
                node.state.clone()
            };
            let state = state.as_ref().borrow();
            let node = {
                let (node, is_new) =
                    init_member(cont.clone(), InitType::Child, |this| View::new(this));
                View::render(node.clone());
                node
            };
            {
                let cont = {
                    let node_borrow = node.as_ref().borrow();
                    let node_borrow = node_borrow
                        .as_node()
                        .as_any()
                        .downcast_ref::<View>()
                        .unwrap();
                    if let Some(subst) = node_borrow.get_self_substitute() {
                        subst.upgrade().unwrap()
                    } else {
                        node.clone()
                    }
                };
                let node = {
                    let (node, is_new) =
                        init_member(cont.clone(), InitType::Child, |this| View::new(this));
                    {
                        let mut node = node.as_ref().borrow_mut();
                        let node = node
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<View>()
                            .unwrap();
                        node.orientation(Orientation::Vertical);
                    }
                    View::render(node.clone());
                    node
                };
                {
                    let cont = {
                        let node_borrow = node.as_ref().borrow();
                        let node_borrow = node_borrow
                            .as_node()
                            .as_any()
                            .downcast_ref::<View>()
                            .unwrap();
                        if let Some(subst) = node_borrow.get_self_substitute() {
                            subst.upgrade().unwrap()
                        } else {
                            node.clone()
                        }
                    };
                    let node = {
                        let (node, is_new) =
                            init_member(cont.clone(), InitType::Child, |this| Button::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<Button>()
                                .unwrap();
                            if is_new {
                                node.label("Inc");
                                {
                                    let state_clone = Rc::clone(&this);
                                    node.on_click(move || {
                                        Self::update(state_clone.clone(), Msg::INC)
                                    });
                                }
                            }
                        }
                        Button::render(node.clone());
                        node
                    };
                    {}
                    Button::render(node.clone());
                    let node = {
                        let (node, is_new) =
                            init_member(node.clone(), InitType::Sibling, |this| Button::new(this));
                        {
                            let mut node = node.as_ref().borrow_mut();
                            let node = node
                                .as_node_mut()
                                .as_any_mut()
                                .downcast_mut::<Button>()
                                .unwrap();
                            if is_new {
                                node.label("Dec");
                                {
                                    let state_clone = Rc::clone(&this);
                                    node.on_click(move || {
                                        Self::update(state_clone.clone(), Msg::DEC)
                                    });
                                }
                            }
                        }
                        Button::render(node.clone());
                        node
                    };
                    {}
                    Button::render(node.clone());
                }
                View::render(node.clone());
                let node = {
                    let (node, is_new) =
                        init_member(node.clone(), InitType::Sibling, |this| Text::new(this));
                    {
                        let mut node = node.as_ref().borrow_mut();
                        let node = node
                            .as_node_mut()
                            .as_any_mut()
                            .downcast_mut::<Text>()
                            .unwrap();
                        node.label(&state.count.to_string());
                    }
                    Text::render(node.clone());
                    node
                };
                {}
                Text::render(node.clone());
            }
            View::render(node.clone());
        }
    }
    impl Container for Counter {
        fn get_child(&self) -> &Option<StrongNodeType> {
            &self.child
        }
        fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
            &mut self.child
        }
        fn as_container(&self) -> &dyn Container {
            self
        }
        fn as_container_mut(&mut self) -> &mut dyn Container {
            self
        }
    }
    impl ComponentNode for Counter {
        fn get_self_substitute(&self) -> &Option<WeakNodeType> {
            &self.self_substitute
        }
        fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
            &mut self.self_substitute
        }
        fn is_dirty(&self) -> bool {
            self.is_dirty
        }
        fn mark_dirty(&mut self) {
            self.is_dirty = true
        }
        fn mark_clean(&mut self) {
            self.is_dirty = false
        }
    }
    impl Counter {
        fn update(this: StrongNodeType, msg: Msg) {
            let state = {
                let state_borrow = this.as_ref().borrow();
                let state = state_borrow
                    .as_node()
                    .as_any()
                    .downcast_ref::<Self>()
                    .unwrap();
                state.state.clone()
            };
            let render = {
                let this = Rc::clone(&this);
                move || {
                    let this = Rc::clone(&this);
                    {
                        let mut this_borrow = this.as_ref().borrow_mut();
                        if let GxiNodeType::Component(this) = this_borrow.deref_mut() {
                            this.mark_dirty();
                        }
                    }
                    Self::render(this);
                }
            };
            fn update<F: Fn() + 'static>(
                state: State,
                msg: Msg,
                render: F,
            ) -> AsyncResult<ShouldRender> {
                let mut state = state.as_ref().borrow_mut();
                match msg {
                    Msg::INC => state.count += 1,
                    _ => {
                        if state.count > 0 {
                            state.count -= 1;
                        } else {
                            return Ok(ShouldRender::No);
                        }
                    }
                }
                Ok(ShouldRender::Yes)
            }
            if let ShouldRender::Yes = update(state, msg, render).unwrap() {
                {
                    let mut this_borrow = this.as_ref().borrow_mut();
                    if let GxiNodeType::Component(this) = this_borrow.deref_mut() {
                        this.mark_dirty();
                    }
                }
                Self::render(this);
            }
        }
    }
    impl Counter {
        pub fn count(&mut self, count: Option<u32>) {
            if let Some(count) = count {
                {
                    let mut state = self.state.as_ref().borrow_mut();
                    state.count = count;
                }
                self.mark_dirty();
            }
        }
    }
}
fn main() {
    run::<App>();
}
