use std::ops::{Deref, DerefMut};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsCast;

use crate::{generate_attr, generate_on_func};

pub struct WebContainerWrapper(pub web_sys::Element);

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
    /// # Safety
    ///
    /// may cause undefined behaviour.
    pub unsafe fn inner_html(&self, str: &str) {
        self.0.set_inner_html(str)
    }

    //**************************************** HTML Attributes ****************************************
    //https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
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

    //non global attrs

    //TODO: check for bool attr

    generate_attr!(accept);
    generate_attr!(accept_charset);
    generate_attr!(action);
    generate_attr!(align);
    generate_attr!(allow);
    generate_attr!(alt);
    generate_attr!(r#async);
    generate_attr!(autocomplete);
    generate_attr!(autofocus);
    generate_attr!(autoplay);
    generate_attr!(background);
    generate_attr!(bgcolor);
    generate_attr!(border);
    generate_attr!(buffered);
    generate_attr!(capture);
    generate_attr!(challenge);
    generate_attr!(charset);
    generate_attr!(checked);
    generate_attr!(cite);
    generate_attr!(code);
    generate_attr!(codebase);
    generate_attr!(color);
    generate_attr!(cols);
    generate_attr!(colspan);
    generate_attr!(content);
    generate_attr!(contextmenu);
    generate_attr!(controls);
    generate_attr!(coords);
    generate_attr!(crossorigin);
    generate_attr!(csp);
    generate_attr!(data);
    generate_attr!(data_star);
    generate_attr!(datetime);
    generate_attr!(decoding);
    generate_attr!(default);
    generate_attr!(defer);
    generate_attr!(dirname);
    generate_attr!(disabled);
    generate_attr!(download);
    generate_attr!(enctype);
    generate_attr!(r#for);
    generate_attr!(form);
    generate_attr!(formaction);
    generate_attr!(formenctype);
    generate_attr!(formmethod);
    generate_attr!(formnovalidate);
    generate_attr!(formtarget);
    generate_attr!(headers);
    generate_attr!(height);
    generate_attr!(high);
    generate_attr!(href);
    generate_attr!(hreflang);
    generate_attr!(http_equiv);
    generate_attr!(icon);
    generate_attr!(importance);
    generate_attr!(integrity);
    generate_attr!(intrinsicsize);
    generate_attr!(ismap);
    generate_attr!(keytype);
    generate_attr!(kind);
    generate_attr!(label);
    generate_attr!(language);
    generate_attr!(loading);
    generate_attr!(list);
    generate_attr!(r#loop);
    generate_attr!(low);
    generate_attr!(manifest);
    generate_attr!(max);
    generate_attr!(maxlength);
    generate_attr!(minlength);
    generate_attr!(media);
    generate_attr!(method);
    generate_attr!(min);
    generate_attr!(multiple);
    generate_attr!(muted);
    generate_attr!(name);
    generate_attr!(novalidate);
    generate_attr!(open);
    generate_attr!(optimum);
    generate_attr!(pattern);
    generate_attr!(ping);
    generate_attr!(placeholder);
    generate_attr!(poster);
    generate_attr!(preload);
    generate_attr!(radiogroup);
    generate_attr!(readonly);
    generate_attr!(referrerpolicy);
    generate_attr!(rel);
    generate_attr!(required);
    generate_attr!(reversed);
    generate_attr!(rows);
    generate_attr!(rowspan);
    generate_attr!(sandbox);
    generate_attr!(scope);
    generate_attr!(selected);
    generate_attr!(shape);
    generate_attr!(size);
    generate_attr!(sizes);
    generate_attr!(span);
    generate_attr!(src);
    generate_attr!(srcdoc);
    generate_attr!(srclang);
    generate_attr!(srcset);
    generate_attr!(start);
    generate_attr!(step);
    generate_attr!(summary);
    generate_attr!(target);
    generate_attr!(r#type);
    generate_attr!(usemap);
    generate_attr!(value);
    generate_attr!(width);
    generate_attr!(wrap);

    //*************************************** Event Handlers ***************************************

    // Assigns the closure f to the the given event
    pub fn on<T: 'static + FromWasmAbi, F: FnMut(T) + 'static>(&self, event: &str, f: F) {
        let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut(_)>);
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
