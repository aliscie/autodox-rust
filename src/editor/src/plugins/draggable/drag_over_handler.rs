use super::draggable::Draggable;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Document, window, MouseEvent};
use wasm_bindgen::JsCast;

impl Draggable {
    fn drag_position_handler_function(element: Element, event: MouseEvent) {
        let rec = element.get_bounding_client_rect();
        let y = event.client_y() as i64;
        let element_top = rec.top() as i64;
        let element_h = rec.height() as i64;
        let mut middle_pont = element_h / 2;
        middle_pont = middle_pont.abs();
        let targeted = Draggable::get_targeted(element.clone()).unwrap();

        let drag_icon_width = 20 as f32;
        let dropped_y = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("dragged_e_y").unwrap().unwrap();
        let dropped_x = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("dragged_e_x").unwrap().unwrap();
        let dropped_element = Draggable::get_element_from_point(dropped_x.parse::<f32>().unwrap() + drag_icon_width, dropped_y.parse::<f32>().unwrap()).unwrap();
        if dropped_element == element {
            return;
        }
        let is_below: bool = (rec.bottom() - 10 as f64) <= event.client_y() as f64;
        // let  is_below:bool = (y - (element_top - middle_pont)) > 31
        if is_below {
            targeted.set_attribute("drag_below", "true");
            targeted.remove_attribute("drag_on");
            targeted.remove_attribute("drag_right");
        } else if (rec.right() - 10 as f64) <= (event.page_x() as f64) {
            targeted.set_attribute("drag_right", "true");
            targeted.remove_attribute("drag_on");
            targeted.remove_attribute("drag_below");
        } else {
            targeted.set_attribute("drag_on", "true");
            targeted.remove_attribute("drag_right");
            targeted.remove_attribute("drag_below");
        }
    }

    pub fn get_element_from_point(mut x: f32, y: f32) -> Option<Element> {
        let doc: Document = window().unwrap().document().unwrap();
        let middle_line = doc.query_selector("body").unwrap().unwrap().get_bounding_client_rect().width() as f32 / 2 as f32;
        //TODO
        // This will cause issue when dragging horizontal="true" elements
        if x > middle_line {
            x -= 20 as f32;
        } else {
            x += 20 as f32;
        }
        let curr = doc.element_from_point(x, y);
        return curr;
    }


    pub fn drag_over_handler(&mut self) {
        let mut prev = None;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
            let y_value = event.page_y();
            let x_value = event.page_x();

            let curr = Draggable::get_element_from_point(x_value as f32, y_value as f32).unwrap();


            if prev != Some(curr.get_bounding_client_rect().top()) && prev != None {
                let prev_element = Draggable::get_element_from_point(x_value as f32, prev.unwrap() as f32).unwrap();
                let prev_targeted = Draggable::get_targeted(prev_element.clone()).unwrap();
                prev_targeted.remove_attribute("drag_on").unwrap();
                prev_targeted.remove_attribute("drag_right").unwrap();
                prev_targeted.remove_attribute("drag_below").unwrap();
            }
            prev = Some(curr.get_bounding_client_rect().top());
            let type_: &str = &curr.node_name()[..];

            if !["DIV"].contains(&type_) {
                Draggable::drag_position_handler_function(curr, event);
            }
        }) as Box<dyn FnMut(_)>);

        self.editor.add_event_listener_with_callback("dragover", &closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

