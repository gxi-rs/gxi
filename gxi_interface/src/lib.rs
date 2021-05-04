#[cfg(all(feature = "web", feature = "desktop"))]
compile_error!("Cannot enable both `web` and `desktop` features.");

#[cfg(not(any(feature = "web", feature = "desktop")))]
compile_error!("Either `web` or `desktop` feature must be enabled.");

macro_rules! transparent_block {( $($tt:tt)* ) => ( $($tt)* )}

#[cfg(any(feature = "web", feature = "desktop"))]
#[cfg(not(all(feature = "web", feature = "desktop")))]
transparent_block! {
    mod should_render;
    mod nodes;

    #[cfg(feature = "desktop")]
    pub use gtk;
    #[cfg(feature = "web")]
    pub use wasm_bindgen;
    #[cfg(feature = "web")]
    pub use web_sys;

    pub use gxi_macro::gxi;
    pub use gxi_parsers::{comp_new, comp_state};
    pub use nodes::*;
    pub use should_render::*;

    pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
}
