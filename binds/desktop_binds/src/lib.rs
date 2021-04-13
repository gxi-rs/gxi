pub use containers::*;
use rust_gui_interface::*;
pub use widgets::*;

mod containers;
mod widgets;

//these 2 can be transformed using cfg macros for different platforms
pub type NativeWidget = gtk::Widget;
pub type NativeWidgetContainer = gtk::Container;
