use crate::data::schema::SchemaData;
use crate::engine::scheduler::schedule_resources;
use serde::{Deserialize, Serialize};
use yew::services::ConsoleService;
use yew::worker::*;

pub struct Worker {
    console: ConsoleService,
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerRequest {
    Schedule(SchemaData),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerResponse {
    Scheduled(SchemaData),
}

pub struct Msg {}

impl Agent for Worker {
    // Available:
    // - `Job` (one per bridge on the main thread)
    // - `Context` (shared in the main thread)
    // - `Private` (one per bridge in a separate thread)
    // - `Public` (shared in a separate thread)
    type Reach = Context;
    type Message = Msg;
    type Input = WorkerRequest;
    type Output = WorkerResponse;

    // Create an instance with a link to the agent.
    fn create(link: AgentLink<Self>) -> Self {
        let mut obj = Worker { console: ConsoleService::new(), link };
        obj.console.log("Created worker");
        obj
    }

    // Handle inner messages (from callbacks)
    fn update(&mut self, _msg: Self::Message) {
        self.console.log("Update");
    }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            WorkerRequest::Schedule(mut sd) => {
                schedule_resources(&mut sd);
                self.link.response(who, WorkerResponse::Scheduled(sd));
            }
        }
    }
}
