use yew::prelude::*;

pub struct AppBar {
    title: String,
}

pub enum Msg {
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub title: String,
}

impl Component for AppBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        AppBar {
            title: props.title,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <header class="appbar mdc-top-app-bar mdc-top-app-bar--fixed",>
                <div class="mdc-top-app-bar__row",>
                    <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-start",>
                        <button class="mdc-icon-button material-icons mdc-top-app-bar__navigation-icon--unbounded",>{"menu"}</button>
                        <span class="mdc-top-app-bar__title",>{&self.title}</span>
                    </section>
                </div>
            </header>
        }
    }
}

