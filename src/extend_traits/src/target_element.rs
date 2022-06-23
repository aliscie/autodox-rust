use web_sys::{Element, MouseEvent, window};
use wasm_bindgen::{UnwrapThrowExt, JsCast};

pub trait MyNewTrait {
    fn target_element<'a>(&self)-> Option<Element>;

}

impl MyNewTrait for MouseEvent {
    fn target_element(&self) -> Option<Element> {
        let doc = window().unwrap_throw().document().unwrap_throw();
        let x = self.page_x();
        let y = self.page_y();
        doc.element_from_point(x as f32, y as f32)
    }
}
