use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) speaker: String,
    pub(crate) url: String,
}

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub title: String,
    pub videos: Vec<Video>,
}


#[derive(Properties, PartialEq)]
pub struct StateProps {
    pub title: String,
    pub description: String,
    pub id: usize,
    pub url: String,
}


#[function_component(UseState)]
pub fn state(props: &StateProps) -> Html {
    let counter = use_state(|| false);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_e: MouseEvent| counter.set(!*counter))
    };

    html! {
<div class="card mb-3" style="max-width: 18rem;">
<div {onclick}  class="card-title" key={props.id}>{ format!("Title: {}!",props.title) }</div>
  if *counter {
        <div class="card-text" key={props.id}>{ format!("speaker: {}!",props.description) }</div>
    }
<a href={format!("{}",props.url)} class="btn btn-primary" key={props.id}>{ "watch now"}</a>
</div>
}
}

#[function_component(Videos)]
pub fn videos() -> Html {
    html! {
    <>
<h1>{"sxx"}</h1>
    </>
    }
}