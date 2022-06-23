use std::convert::TryInto;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Node, window, Range};
use web_sys::console::log_1;

use super::main::Mention;
use crate::plugins::main::EditorPlugin;

impl Mention {
    fn selection() -> Option<usize> {
        let sel = window().unwrap_throw().get_selection().unwrap().unwrap().get_range_at(0).unwrap().end_offset().unwrap();
        Some((sel) as usize)
    }
    pub(crate) fn mention_handler(&mut self) {
        let editor = self.editor.clone();
        let doc = self.doc.clone();
        let mut prev: Option<Element> = None;
        let mut curr: Option<Element> = None;
        let mut i: Option<usize> = None;
        let mut f: Option<usize> = None;

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            prev = curr.clone();
            let range = window().unwrap_throw().get_selection().unwrap().unwrap().get_range_at(0).unwrap();
            curr = Some(range.common_ancestor_container().unwrap().parent_element().unwrap());
            if prev != curr {
                i = None;
            }
            if event.key() == "@" {
                i = Mention::selection();
                Mention::make_menu( &curr.as_ref().unwrap(), &doc.as_ref());
            }

            if i != None {
                f = Mention::selection();
                let text = &curr.as_ref().unwrap().text_content().unwrap();
                if !curr.as_ref().unwrap().query_selector("#drag").unwrap().is_none() {
                    log_1(&format!("{:#?}", &text[i.unwrap() + 2..f.unwrap() + 2]).into());
                    
                } else { log_1(&format!("{:#?}", &text[i.unwrap()..f.unwrap()]).into()); }
            }
        }) as Box<dyn FnMut(_)>);

        self.editor.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
