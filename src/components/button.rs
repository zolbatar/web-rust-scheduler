use crate::components::style::*;
use yew::prelude::*;

pub struct Button {
    title: String,
    variant: usize,
    onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub title: String,
    pub variant: usize,
    pub onclick: Option<Callback<()>>,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Button { title: props.title, variant: props.variant, onclick: props.onclick }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.onclick {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.variant = props.variant;
        self.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html<Self> {
        let cls = match self.variant {
            VARIANT_FLAT => "mdc-button",
            VARIANT_CONTAINED => "mdc-button mdc-button--raised",
            VARIANT_OUTLINED => "mdc-button mdc-button--outlined",
            _ => "mdc-button",
        };
        html! {
            <button class={cls}, onclick=|_| Msg::Clicked,><span class="mdc-button__ripple",></span>{ &self.title }</button>
        }
    }
}
