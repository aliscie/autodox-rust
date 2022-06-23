use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{Document, Element, window};
use web_sys::console::log_1;

use crate::plugins::main::PluginTraits;

pub struct Mention {
    pub(crate) drag_icon_width: f32,
    pub(crate) doc: Document,
    pub(crate) body: Element,
    pub(crate) editor: Rc<Element>,
    prev: Option<Element>,
    pub(crate) curr: Option<Element>,
    dragged: Option<Element>,
}

impl Mention {
    pub(crate) fn new(editor: Rc<Element>) {
        let drag_icon_width = 20 as f32;
        let doc = window().unwrap_throw().document().unwrap_throw();
        let body = doc.query_selector("body").unwrap_throw().unwrap_throw();
        let _self = &mut Mention { editor, drag_icon_width, doc, body, prev: None, curr: None, dragged: None };
        _self.mention_handler();
    }
}

impl PluginTraits for Mention {}
