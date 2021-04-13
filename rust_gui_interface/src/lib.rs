pub use tokio::*;

pub use c::c;
pub use comp::comp;
pub use nodes::*;
#[macro_use]
pub use nodes::node_macros;
pub use parsers::comp_init;
pub use update::update;

mod nodes;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
