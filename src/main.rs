#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
use crate::components::svgmap::SVGMap;
use crate::data::input::SCENARIO;
use crate::data::schema::SchemaData;
use crate::engine::worker::{Worker, WorkerRequest, WorkerResponse};
use crate::sections::appbar::AppBar;
use crate::sections::infopanel::InfoPanel;
use crate::timeout::create_timeout;
use ron::de::from_str;
use std::collections::HashMap;
use std::time::Duration;
use yew::prelude::*;
use yew::services::ConsoleService;

mod components;
mod data;
mod engine;
mod map;
mod sections;
mod timeout;

pub struct Model {
    console: ConsoleService,
    link: ComponentLink<Self>,
    context: Box<dyn Bridge<Worker>>,
    task_count: usize,
    worker_count: usize,
    data: SchemaData,
    zoom: bool,
    day: usize,
}

pub enum Msg {
    ContextMsg(WorkerResponse),
    RequestNewSchedule,
    ZoomChanged,
    DayChanged(usize),
}

pub const NBSP: char = '\u{00a0}';

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        // Set up the worker thread
        let callback = link.send_back(Msg::ContextMsg);

        // Create initial object with empty values, we do this to get access to the console.log
        let mut obj = Model {
            console: ConsoleService::new(),
            link,
            context: Worker::bridge(callback), // Connected! :tada:
            task_count: 0,
            worker_count: 0,
            data: SchemaData { activity: HashMap::new(), resource: HashMap::new(), route: Vec::new(), allocation: HashMap::new(), status: None },
            zoom: true,
            day: 0,
        };

        // Create schema
        obj.console.log("Loading schema data");
        obj.data = match from_str(SCENARIO) {
            Ok(x) => x,
            Err(e) => {
                obj.console.log(&format!("Failed to load config: {}", e));
                obj.data
            }
        };
        obj.console.log(&format!("Creating schedule for {} resources and {} activities", obj.data.resource.len(), obj.data.activity.len()));
        obj.task_count = obj.data.activity.len();
        obj.worker_count = obj.data.resource.len();

        // Our model defines the state
        obj
    }

    fn mounted(&mut self) -> ShouldRender {
        // Request the data to be scheduled
        (*self.context).send(WorkerRequest::Schedule(self.data.clone()));

        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ContextMsg(response) => match response {
                WorkerResponse::Scheduled(sd) => {
                    self.data = sd;
                    if self.data.status.as_ref().unwrap().changed {
                        self.console.log("New plan returned from scheduler!");
                    }

                    // Request another reschedule, we want to do this forever to show the improving schedule
                    // We wait a little while so we don't swamp the main thread
                    // Unfortunately it CAN'T run on another thread yet
                    let callback = self.link.send_back(|_| Msg::RequestNewSchedule);
                    create_timeout(Duration::from_millis(250), callback);

                    // Ask for a redraw
                    return true;
                }
            },
            Msg::RequestNewSchedule => {
                (*self.context).send(WorkerRequest::Schedule(self.data.clone()));
                return false;
            }
            Msg::ZoomChanged => {
                self.zoom = !self.zoom;
                return true;
            }
            Msg::DayChanged(day) => {
                self.day = day;
                return true;
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let status = self.data.status.as_ref().unwrap();
        html! {
            <div class="mdc-typography",>
                <div class="full",>
                    <AppBar: title="Cognito iQ Web Scheduler", />
                    <InfoPanel: num_jobs={&self.task_count}, 
                                num_workers={&self.worker_count}, 
                                dist={status.distance}, 
                                value={status.value}, 
                                travel_time={status.travel_time}, 
                                quality={status.quality},
                                zoom_changed=|_| Msg::ZoomChanged day_changed=|day| Msg::DayChanged(day),/>
                    <div class="centre-all",>
                    <SVGMap: data={&self.data}, zoom={&self.zoom}, background="#121212", day={&self.day} />
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
