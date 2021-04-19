use rust_gui_interface::{Node, web_sys};
use rust_gui_interface::wasm_bindgen::JsCast;

use crate::Closure;

pub trait GlobalAttributes: Node {
    fn on<F: Fn() + 'static>(&self, event: &str, f: F) {
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            f();
        }) as Box<dyn Fn(_)>);
        self.get_widget().add_event_listener_with_callback(event, closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    #[inline]
    fn inner_html(&self, str: &str) {
        self.get_widget().set_inner_html(str)
    }

    #[inline]
    fn on_abort<F: Fn() + 'static>(&self, f: F) {
        self.on("abort", f);
    }


    #[inline]
    fn on_autocomplete<F: Fn() + 'static>(&self, f: F) {
        self.on("autocomplete", f);
    }


    #[inline]
    fn on_autocompleteerror<F: Fn() + 'static>(&self, f: F) {
        self.on("autocompleteerror", f);
    }


    #[inline]
    fn on_blur<F: Fn() + 'static>(&self, f: F) {
        self.on("blur", f);
    }


    #[inline]
    fn on_cancel<F: Fn() + 'static>(&self, f: F) {
        self.on("cancel", f);
    }


    #[inline]
    fn on_canplay<F: Fn() + 'static>(&self, f: F) {
        self.on("canplay", f);
    }


    #[inline]
    fn on_canplaythrough<F: Fn() + 'static>(&self, f: F) {
        self.on("canplaythrough", f);
    }


    #[inline]
    fn on_change<F: Fn() + 'static>(&self, f: F) {
        self.on("change", f);
    }


    #[inline]
    fn on_click<F: Fn() + 'static>(&self, f: F) {
        self.on("click", f);
    }


    #[inline]
    fn on_close<F: Fn() + 'static>(&self, f: F) {
        self.on("close", f);
    }


    #[inline]
    fn on_contextmenu<F: Fn() + 'static>(&self, f: F) {
        self.on("contextmenu", f);
    }


    #[inline]
    fn on_cuechange<F: Fn() + 'static>(&self, f: F) {
        self.on("cuechange", f);
    }


    #[inline]
    fn on_dblclick<F: Fn() + 'static>(&self, f: F) {
        self.on("dblclick", f);
    }


    #[inline]
    fn on_drag<F: Fn() + 'static>(&self, f: F) {
        self.on("drag", f);
    }


    #[inline]
    fn on_dragend<F: Fn() + 'static>(&self, f: F) {
        self.on("dragend", f);
    }


    #[inline]
    fn on_dragenter<F: Fn() + 'static>(&self, f: F) {
        self.on("dragenter", f);
    }


    #[inline]
    fn on_dragexit<F: Fn() + 'static>(&self, f: F) {
        self.on("dragexit", f);
    }


    #[inline]
    fn on_dragleave<F: Fn() + 'static>(&self, f: F) {
        self.on("dragleave", f);
    }


    #[inline]
    fn on_dragover<F: Fn() + 'static>(&self, f: F) {
        self.on("dragover", f);
    }


    #[inline]
    fn on_dragstart<F: Fn() + 'static>(&self, f: F) {
        self.on("dragstart", f);
    }


    #[inline]
    fn on_drop<F: Fn() + 'static>(&self, f: F) {
        self.on("drop", f);
    }


    #[inline]
    fn on_durationchange<F: Fn() + 'static>(&self, f: F) {
        self.on("durationchange", f);
    }


    #[inline]
    fn on_emptied<F: Fn() + 'static>(&self, f: F) {
        self.on("emptied", f);
    }


    #[inline]
    fn on_ended<F: Fn() + 'static>(&self, f: F) {
        self.on("ended", f);
    }


    #[inline]
    fn on_error<F: Fn() + 'static>(&self, f: F) {
        self.on("error", f);
    }


    #[inline]
    fn on_focus<F: Fn() + 'static>(&self, f: F) {
        self.on("focus", f);
    }


    #[inline]
    fn on_input<F: Fn() + 'static>(&self, f: F) {
        self.on("input", f);
    }


    #[inline]
    fn on_invalid<F: Fn() + 'static>(&self, f: F) {
        self.on("invalid", f);
    }


    #[inline]
    fn on_keydown<F: Fn() + 'static>(&self, f: F) {
        self.on("keydown", f);
    }


    #[inline]
    fn on_keypress<F: Fn() + 'static>(&self, f: F) {
        self.on("keypress", f);
    }


    #[inline]
    fn on_keyup<F: Fn() + 'static>(&self, f: F) {
        self.on("keyup", f);
    }


    #[inline]
    fn on_load<F: Fn() + 'static>(&self, f: F) {
        self.on("load", f);
    }


    #[inline]
    fn on_loadeddata<F: Fn() + 'static>(&self, f: F) {
        self.on("loadeddata", f);
    }


    #[inline]
    fn on_loadedmetadata<F: Fn() + 'static>(&self, f: F) {
        self.on("loadedmetadata", f);
    }


    #[inline]
    fn on_loadstart<F: Fn() + 'static>(&self, f: F) {
        self.on("loadstart", f);
    }


    #[inline]
    fn on_mousedown<F: Fn() + 'static>(&self, f: F) {
        self.on("mousedown", f);
    }


    #[inline]
    fn on_mouseenter<F: Fn() + 'static>(&self, f: F) {
        self.on("mouseenter", f);
    }


    #[inline]
    fn on_mouseleave<F: Fn() + 'static>(&self, f: F) {
        self.on("mouseleave", f);
    }


    #[inline]
    fn on_mousemove<F: Fn() + 'static>(&self, f: F) {
        self.on("mousemove", f);
    }


    #[inline]
    fn on_mouseout<F: Fn() + 'static>(&self, f: F) {
        self.on("mouseout", f);
    }


    #[inline]
    fn on_mouseover<F: Fn() + 'static>(&self, f: F) {
        self.on("mouseover", f);
    }


    #[inline]
    fn on_mouseup<F: Fn() + 'static>(&self, f: F) {
        self.on("mouseup", f);
    }


    #[inline]
    fn on_mousewheel<F: Fn() + 'static>(&self, f: F) {
        self.on("mousewheel", f);
    }


    #[inline]
    fn on_pause<F: Fn() + 'static>(&self, f: F) {
        self.on("pause", f);
    }


    #[inline]
    fn on_play<F: Fn() + 'static>(&self, f: F) {
        self.on("play", f);
    }


    #[inline]
    fn on_playing<F: Fn() + 'static>(&self, f: F) {
        self.on("playing", f);
    }


    #[inline]
    fn on_progress<F: Fn() + 'static>(&self, f: F) {
        self.on("progress", f);
    }


    #[inline]
    fn on_ratechange<F: Fn() + 'static>(&self, f: F) {
        self.on("ratechange", f);
    }


    #[inline]
    fn on_reset<F: Fn() + 'static>(&self, f: F) {
        self.on("reset", f);
    }


    #[inline]
    fn on_resize<F: Fn() + 'static>(&self, f: F) {
        self.on("resize", f);
    }


    #[inline]
    fn on_scroll<F: Fn() + 'static>(&self, f: F) {
        self.on("scroll", f);
    }


    #[inline]
    fn on_seeked<F: Fn() + 'static>(&self, f: F) {
        self.on("seeked", f);
    }


    #[inline]
    fn on_seeking<F: Fn() + 'static>(&self, f: F) {
        self.on("seeking", f);
    }


    #[inline]
    fn on_select<F: Fn() + 'static>(&self, f: F) {
        self.on("select", f);
    }


    #[inline]
    fn on_show<F: Fn() + 'static>(&self, f: F) {
        self.on("show", f);
    }


    #[inline]
    fn on_sort<F: Fn() + 'static>(&self, f: F) {
        self.on("sort", f);
    }


    #[inline]
    fn on_stalled<F: Fn() + 'static>(&self, f: F) {
        self.on("stalled", f);
    }


    #[inline]
    fn on_submit<F: Fn() + 'static>(&self, f: F) {
        self.on("submit", f);
    }


    #[inline]
    fn on_suspend<F: Fn() + 'static>(&self, f: F) {
        self.on("suspend", f);
    }


    #[inline]
    fn on_timeupdate<F: Fn() + 'static>(&self, f: F) {
        self.on("timeupdate", f);
    }


    #[inline]
    fn on_toggle<F: Fn() + 'static>(&self, f: F) {
        self.on("toggle", f);
    }


    #[inline]
    fn on_volumechange<F: Fn() + 'static>(&self, f: F) {
        self.on("volumechange", f);
    }


    #[inline]
    fn on_waiting<F: Fn() + 'static>(&self, f: F) {
        self.on("waiting", f);
    }
}
