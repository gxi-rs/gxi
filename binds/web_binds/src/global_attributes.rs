use rust_gui_interface::wasm_bindgen::JsCast;
use rust_gui_interface::{web_sys, Node};

use crate::Closure;

macro_rules! generate_on_func {
    ($name:ident $event:literal) => {
        #[inline]
        fn $name<F: Fn() + 'static>(&self, f: F) {
            self.on($event, f);
        }
    };
}

pub trait GlobalAttributes: Node {
    #[inline]
    fn accesskey(&self, key: &str) {
        self.get_widget().set_attribute("accesskey", key);
    }

    #[inline]
    fn inner_html(&self, str: &str) {
        self.get_widget().set_inner_html(str)
    }

    fn on<F: Fn() + 'static>(&self, event: &str, f: F) {
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            f();
        }) as Box<dyn Fn(_)>);
        self.get_widget()
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    generate_on_func!(on_abort "abort");

    generate_on_func!(on_autocomplete "autocomplete");

    generate_on_func!(on_autocompleteerror "autocompleteerror");

    generate_on_func!(on_blur "blur");

    generate_on_func!(on_cancel "cancel");

    generate_on_func!(on_canplay "canplay");

    generate_on_func!(on_canplaythrough "canplaythrough");

    generate_on_func!(on_change "change");

    generate_on_func!(on_click "click");

    generate_on_func!(on_close "close");

    generate_on_func!(on_contextmenu "contextmenu");

    generate_on_func!(on_cuechange "cuechange");

    generate_on_func!(on_dblclick "dblclick");

    generate_on_func!(on_drag "drag");

    generate_on_func!(on_dragend "dragend");

    generate_on_func!(on_dragenter "dragenter");

    generate_on_func!(on_dragexit "dragexit");

    generate_on_func!(on_dragleave "dragleave");

    generate_on_func!(on_dragover "dragover");

    generate_on_func!(on_dragstart "dragstart");

    generate_on_func!(on_drop "drop");

    generate_on_func!(on_durationchange "durationchange");

    generate_on_func!(on_emptied "emptied");

    generate_on_func!(on_ended "ended");

    generate_on_func!(on_error "error");

    generate_on_func!(on_focus "focus");

    generate_on_func!(on_input "input");

    generate_on_func!(on_invalid "invalid");

    generate_on_func!(on_keydown "keydown");

    generate_on_func!(on_keypress "keypress");

    generate_on_func!(on_keyup "keyup");

    generate_on_func!(on_load "load");

    generate_on_func!(on_loadeddata "loadeddata");

    generate_on_func!(on_loadedmetadata "loadedmetadata");

    generate_on_func!(on_loadstart "loadstart");

    generate_on_func!(on_mousedown "mousedown");

    generate_on_func!(on_mouseenter "mouseenter");

    generate_on_func!(on_mouseleave "mouseleave");

    generate_on_func!(on_mousemove "mousemove");

    generate_on_func!(on_mouseout "mouseout");

    generate_on_func!(on_mouseover "mouseover");

    generate_on_func!(on_mouseup "mouseup");

    generate_on_func!(on_mousewheel "mousewheel");

    generate_on_func!(on_pause "pause");

    generate_on_func!(on_play "play");

    generate_on_func!(on_playing "playing");

    generate_on_func!(on_progress "progress");

    generate_on_func!(on_ratechange "ratechange");

    generate_on_func!(on_reset "reset");

    generate_on_func!(on_resize "resize");

    generate_on_func!(on_scroll "scroll");

    generate_on_func!(on_seeked "seeked");

    generate_on_func!(on_seeking "seeking");

    generate_on_func!(on_select "select");

    generate_on_func!(on_show "show");

    generate_on_func!(on_sort "sort");

    generate_on_func!(on_stalled "stalled");

    generate_on_func!(on_submit "submit");

    generate_on_func!(on_suspend "suspend");

    generate_on_func!(on_timeupdate "timeupdate");

    generate_on_func!(on_toggle "toggle");

    generate_on_func!(on_volumechange "volumechange");

    generate_on_func!(on_waiting "waiting");
}
