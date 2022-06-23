use web_sys::console::log_1;
use std::rc::Rc;
use web_sys::{Element, MouseEvent,window, Document};
use wasm_bindgen::UnwrapThrowExt;


pub trait EditorPlugin {
    fn remove_menu<'a>(curr: &'a Element, doc: &'a Document);
    fn make_menu<'a>( curr: &'a Element, doc: &'a Document);
}

pub trait PluginTraits {}




impl<T> EditorPlugin for T where T: PluginTraits {

    fn remove_menu<'a>(curr: &'a Element, doc: &'a Document) {
        let my_dropdown = doc.query_selector(".dropdown-content");
        let mut pass = false;
        pass |= my_dropdown == Ok(None);
        pass |= curr.id() == "myDropdown";
        if pass { return; };
        my_dropdown.unwrap().unwrap().remove();
    }

    fn make_menu<'a>( curr: &'a Element, doc: &'a Document) {
        let left = curr.get_bounding_client_rect().left() as f64 + 20 as f64;
        let top = curr.get_bounding_client_rect().top() as f64 - 5 as f64;

        let style = &format!("margin-left: 0; position: absolute; left:{}px; top:{}px", left, top);

        let drop_down_list = r#"
                                        <a id="myDropdown">Home</a>
                                        <a id="myDropdown">About</a>
                                        <a id="myDropdown">Contact</a>
                                        "#;

        let menu_list: Element = doc.create_element("div").unwrap();
        menu_list.set_attribute("id", "dropdown-content ");
        menu_list.set_attribute("class", "dropdown-content ");
        menu_list.set_inner_html(&drop_down_list);
        menu_list.set_attribute("style", style);
        menu_list.set_attribute("contenteditable", "false");
        let res = doc.query_selector("body").unwrap_throw().unwrap_throw().append_child(&menu_list);
    }
}