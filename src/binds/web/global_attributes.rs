use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsCast;

use crate::*;

pub trait GlobalAttributes: WidgetNode {
    fn get_widget_as_element(&self) -> &web_sys::Element;

    #[inline]
    fn inner_html(&self, str: &str) {
        self.get_widget_as_element().set_inner_html(str)
    }

    //Global attributes according to https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
    generate_attr!(accesskey);
    generate_attr!(autocapitalize);
    generate_attr!(class);
    generate_attr!(contenteditable);
    generate_attr!(dir);
    generate_attr!(draggable);
    generate_attr!(enterkeyhint);
    generate_attr!(hidden);
    generate_attr!(id);
    generate_attr!(inputmode);
    generate_attr!(is);
    generate_attr!(itemid);
    generate_attr!(itemprop);
    generate_attr!(itemref);
    generate_attr!(itemscope);
    generate_attr!(itemtype);
    generate_attr!(lang);
    generate_attr!(nonce);
    generate_attr!(part);
    generate_attr!(slot);
    generate_attr!(spellcheck);
    generate_attr!(style);
    generate_attr!(tabindex);
    generate_attr!(title);
    generate_attr!(translate);

    // Assigns the closure f to the the given event
    fn on<T: 'static + FromWasmAbi, F: Fn(T) + 'static>(&self, event: &str, f: F) {
        let closure = Closure::wrap(Box::new(f) as Box<dyn Fn(_)>);
        self.get_widget_as_element()
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    //Global event handlers according to https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
    generate_on_func!(on_abort "abort");
    generate_on_func!(on_autocomplete "autocomplete" );
    generate_on_func!(on_autocompleteerror "autocompleteerror");
    generate_on_func!(on_blur "blur" FocusEvent);
    generate_on_func!(on_cancel "cancel");
    generate_on_func!(on_canplay "canplay");
    generate_on_func!(on_canplaythrough "canplaythrough");
    generate_on_func!(on_change "change");
    generate_on_func!(on_click "click" MouseEvent);
    generate_on_func!(on_close "close");
    generate_on_func!(on_contextmenu "contextmenu" MouseEvent);
    generate_on_func!(on_cuechange "cuechange");
    generate_on_func!(on_dblclick "dblclick" MouseEvent);
    generate_on_func!(on_drag "drag" DragEvent);
    generate_on_func!(on_dragend "dragend" DragEvent);
    generate_on_func!(on_dragenter "dragenter" DragEvent);
    generate_on_func!(on_dragexit "dragexit" DragEvent);
    generate_on_func!(on_dragleave "dragleave" DragEvent);
    generate_on_func!(on_dragover "dragover" DragEvent);
    generate_on_func!(on_dragstart "dragstart" DragEvent);
    generate_on_func!(on_drop "drop" DragEvent);
    generate_on_func!(on_durationchange "durationchange");
    generate_on_func!(on_emptied "emptied");
    generate_on_func!(on_ended "ended");
    generate_on_func!(on_error "error");
    generate_on_func!(on_focus "focus" FocusEvent);
    generate_on_func!(on_input "input" InputEvent);
    generate_on_func!(on_invalid "invalid");
    generate_on_func!(on_keydown "keydown" KeyboardEvent);
    generate_on_func!(on_keypress "keypress" KeyboardEvent);
    generate_on_func!(on_keyup "keyup" KeyboardEvent);
    generate_on_func!(on_load "load");
    generate_on_func!(on_loadeddata "loadeddata");
    generate_on_func!(on_loadedmetadata "loadedmetadata");
    generate_on_func!(on_loadstart "loadstart" ProgressEvent);
    generate_on_func!(on_mousedown "mousedown" MouseEvent);
    generate_on_func!(on_mouseenter "mouseenter" MouseEvent);
    generate_on_func!(on_mouseleave "mouseleave" MouseEvent);
    generate_on_func!(on_mousemove "mousemove" MouseEvent);
    generate_on_func!(on_mouseout "mouseout" MouseEvent);
    generate_on_func!(on_mouseover "mouseover" MouseEvent);
    generate_on_func!(on_mouseup "mouseup" MouseEvent);
    generate_on_func!(on_mousewheel "mousewheel" WheelEvent);
    generate_on_func!(on_pause "pause");
    generate_on_func!(on_play "play");
    generate_on_func!(on_playing "playing");
    generate_on_func!(on_progress "progress" ProgressEvent);
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
    generate_on_func!(on_submit "submit" FocusEvent);
    generate_on_func!(on_suspend "suspend");
    generate_on_func!(on_timeupdate "timeupdate");
    generate_on_func!(on_toggle "toggle");
    generate_on_func!(on_volumechange "volumechange");
    generate_on_func!(on_waiting "waiting");
}
