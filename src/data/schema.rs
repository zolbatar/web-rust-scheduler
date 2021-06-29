use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Activity {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
}

#[allow(dead_code)]
impl Activity {
    fn new(id: String, lat: f64, lon: f64) -> Activity {
        Activity { id, lat, lon }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
}

#[allow(dead_code)]
impl Resource {
    pub fn new(id: String, lat: f64, lon: f64) -> Resource {
        Resource { id, lat, lon }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Allocation {
    pub activity_id: String,
    pub dist: f64,
    pub time: f64,
    pub travel_to: f64,
    pub travel_from: f64,
}

#[allow(dead_code)]
impl Allocation {
    pub fn new(activity_id: String, dist: f64, time: f64, travel_from: f64, travel_to: f64) -> Allocation {
        Allocation { activity_id, dist, time, travel_to, travel_from }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Route {
    pub resource_id: String,
    pub allocation: Vec<String>,
    pub time: f64,
}

#[allow(dead_code)]
impl Route {
    pub fn new(resource_id: String, time: f64) -> Route {
        Route { resource_id, allocation: Vec::new(), time }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct Status {
    pub start_time: f64,
    pub new_data: bool,
    pub changed: bool,
    pub quality: f64,
    pub distance: f64,
    pub value: f64,
    pub travel_time: f64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct SchemaData {
    pub activity: HashMap<String, Activity>,
    pub resource: HashMap<String, Resource>,
    pub allocation: HashMap<String, Allocation>,
    pub route: Vec<Route>,
    pub status: Option<Status>,
}
