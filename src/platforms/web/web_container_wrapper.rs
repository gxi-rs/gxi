use std::ops::{Deref, DerefMut};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsCast;

use crate::{generate_on_func, NativeContainerExt, NativeWidget, WebContainer};

pub struct WebContainerWrapper(pub web_sys::Element);

impl NativeContainerExt for WebContainerWrapper {
    fn append(&mut self, widget: &NativeWidget) {
        self.0.append_child(widget).unwrap();
    }

    fn move_to_index(&mut self, _widget: &NativeWidget, _index: usize) {
        todo!()
    }
}

impl Default for WebContainerWrapper {
    fn default() -> Self {
        "div".into()
    }
}

impl From<&str> for WebContainerWrapper {
    fn from(name: &str) -> Self {
        Self({
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            document.create_element(name).unwrap()
        })
    }
}

impl Deref for WebContainerWrapper {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WebContainerWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// add extra calls

impl WebContainerWrapper {
    pub fn inner_html(&self, str: &str) {
        self.0.set_inner_html(str)
    }

    //*************************************** Event Handlers ***************************************

    // Assigns the closure f to the the given event
    pub fn on<T: 'static + FromWasmAbi, F: Fn(T) + 'static>(&self, event: &str, f: F) {
        let closure = Closure::wrap(Box::new(f) as Box<dyn Fn(_)>);
        self.0
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

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
