use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, MouseEvent, window, Document};
use web_sys::console::log_1;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}


#[function_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {

    let doc = window().unwrap_throw().document().unwrap_throw();
    let body: &Element = &doc.query_selector("body").unwrap_throw().unwrap_throw();
    let window_width = window().unwrap_throw().inner_width().unwrap_throw();


    let mousemove_closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let my_sidenav: &Element = &doc.query_selector("#mySidenav").unwrap().unwrap();
        let pagex = e.page_x().to_string();

        let my_sidenav_width = my_sidenav.client_width().to_string();

        if pagex.parse::<i32>().unwrap() < 25 {
            // if so, open sidenav
            my_sidenav.set_attribute("style", "width:250px;");

        } else if(pagex.parse::<i32>().unwrap() > my_sidenav_width.parse::<i32>().unwrap_throw()){
            // if not, close sidenav
            my_sidenav.set_attribute("style", "width:0px;");
        };

    }) as Box<dyn FnMut(_)>);
    &body.add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref()).unwrap_throw();
    mousemove_closure.forget();

    html! {
        <div style="width:0" id="mySidenav" class="sidenav">
        { props.children.clone() }
        </div>
    }
}