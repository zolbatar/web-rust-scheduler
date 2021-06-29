use crate::components::button::Button;
use crate::components::style::*;
use crate::NBSP;
use yew::prelude::*;

pub struct InfoPanel {
    num_jobs: usize,
    num_workers: usize,
    dist: f64,
    value: f64,
    travel_time: f64,
    quality: f64,
    day: usize,
    zoom_changed: Option<Callback<()>>,
    day_changed: Option<Callback<usize>>,
}

pub enum Msg {
    ZoomChanged,
    DayChanged(usize),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub num_jobs: usize,
    pub num_workers: usize,
    pub dist: f64,
    pub value: f64,
    pub travel_time: f64,
    pub quality: f64,
    pub day: usize,
    pub zoom_changed: Option<Callback<()>>,
    pub day_changed: Option<Callback<usize>>,
}

impl Component for InfoPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        InfoPanel {
            num_jobs: props.num_jobs,
            num_workers: props.num_workers,
            dist: props.dist,
            value: props.value,
            travel_time: props.travel_time,
            zoom_changed: props.zoom_changed,
            day_changed: props.day_changed,
            quality: props.quality,
            day: props.day,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ZoomChanged => {
                if let Some(ref mut callback) = self.zoom_changed {
                    callback.emit(());
                }
            }
            Msg::DayChanged(day) => {
                if let Some(ref mut callback) = self.day_changed {
                    callback.emit(day);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.num_jobs = props.num_jobs;
        self.num_workers = props.num_workers;
        self.dist = props.dist;
        self.value = props.value;
        self.travel_time = props.travel_time;
        self.quality = props.quality;
        self.day = props.day;
        true
    }

    fn view(&self) -> Html<Self> {
        let tasks = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Task Count:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{&self.num_jobs}</span>
            </div>
        };

        let workers = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Worker Count:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{&self.num_workers}</span>
            </div>
        };

        let dist = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Distance:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{self.dist as i64 / 1000}{" km"}</span>
            </div>
        };

        let avg = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Average:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{format!("{:.2}", self.dist / 1000.0 / 500.0)}{" km"}</span>
            </div>
        };

        let travel = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Travel:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{format!("{:.2}", self.travel_time / 60.0)}{" minutes"}</span>
            </div>
        };

        let value = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Value:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{"£"}{format!("{:.2}", self.value)}</span>
            </div>
        };

        let quality = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Quality:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{format!("{:.2}", self.quality)}{"%"}</span>
            </div>
        };

        let day = html! {
            <div>
                <span class="mdc-typography--body1",>{ "Day:" }{NBSP}</span>
                <span class="mdc-typography--headline6",>{format!("{}", self.day)}</span>
            </div>
        };

        html! {
            <div class="info-panel", style="clear:all",>
                {tasks}
                <br />
                {workers}
                <br />
                {dist}
                <br />
                {avg}
                <br />
                {travel}
                <br />
                {value}
                <br />
                {quality}
                <br />
                <br />
                {day}
                <Button: onclick=|_| Msg::ZoomChanged, title="Flip Zoom", variant=VARIANT_OUTLINED, />
                <Button: onclick=|_| Msg::DayChanged(0), title="Day 0", variant=VARIANT_OUTLINED, />
                <Button: onclick=|_| Msg::DayChanged(1), title="Day 1", variant=VARIANT_OUTLINED, />
                <Button: onclick=|_| Msg::DayChanged(2), title="Day 2", variant=VARIANT_OUTLINED, />
                <Button: onclick=|_| Msg::DayChanged(3), title="Day 3", variant=VARIANT_OUTLINED, />
                <Button: onclick=|_| Msg::DayChanged(4), title="Day 4", variant=VARIANT_OUTLINED, />
                <Button: onclick=|_| Msg::DayChanged(5), title="Day 5", variant=VARIANT_OUTLINED, />
            </div>
        }
    }
}
