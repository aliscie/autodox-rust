use std::convert::TryInto;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Node, window};
use web_sys::console::log_1;

use super::main::TableFormula;

impl TableFormula {
    fn parse_formula(string: &Option<String>) -> String {
        let mut formula = string.clone().unwrap();
        formula.remove(0);
        let mut pares = format!("actual value  of {:?}", formula);
        pares
    }
    fn is_formula(string: &Option<String>) -> bool {
        let first_chr = string.as_ref().unwrap().trim().chars().nth(0).unwrap();
        first_chr == '='
    }

    fn get_parent<'a>(curr: &'a Element, t: &'a str, tag: &'a str) -> Element {
        let mut parent = curr.clone();
        if curr.tag_name() == tag {
            while parent.tag_name() != t {
                parent = parent.parent_element().unwrap();
            }
        }
        parent
    }

    fn get_table(curr: &Element) {
        let html = TableFormula::get_parent(curr, "TABLE", "TD");

        let table = table_extract::Table::find_first(&&html.outer_html()).unwrap();
        for row in &table {
            log_1(&format!(
                "{},___|__ {}",
                row.get("Firstname").unwrap_or("<name missing>"),
                row.get("Lastname").unwrap_or("<age missing>")
            ).into());
        }
    }

    pub(crate) fn formula_handler(&mut self) {
        let mut prev: Option<Element> = None;
        let mut curr: Option<Element> = None;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            prev = curr.clone();
            let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
            //TODO
            // When I click on concatenated=false item I got `get_range_at(0)` not valid index error

            let range = window().unwrap_throw().get_selection().unwrap().unwrap();


            let range = range.get_range_at(0).unwrap();//.unwrap_or(return);
            curr = range.common_ancestor_container().unwrap().parent_element();
            let table = TableFormula::get_table(curr.as_ref().unwrap());
            log_1(&format!("{:#?}", table).into());

            let curr_formula = curr.as_ref().unwrap().get_attribute("formula");
            if prev == curr { return; }
            if format!("{:?}", prev) != "None" {
                let prev_text_content = prev.as_ref().unwrap().text_content();

                if TableFormula::is_formula(&prev_text_content) {
                    let formula = &prev_text_content.as_ref().unwrap();
                    let parsed_formula = TableFormula::parse_formula(&prev_text_content);

                    prev.as_ref().unwrap().set_attribute("formula", &formula);
                    prev.as_ref().unwrap().set_text_content(Some(&parsed_formula as &str));
                } else if prev.as_ref().unwrap().has_attribute("formula") {
                    prev.as_ref().unwrap().remove_attribute("formula");
                }
            }
            if curr_formula != None {
                curr.as_ref().unwrap().set_text_content(Some(&curr_formula.unwrap() as &str))
            }
        }) as Box<dyn
        FnMut(_)>);

        self.editor.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
        self.editor.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
