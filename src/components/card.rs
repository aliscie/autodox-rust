use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <div class="card">
        { props.children.clone() }
        </div>
    }
}
