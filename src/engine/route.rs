use crate::data::schema::{Allocation, Route, SchemaData};
use crate::engine::travel::{distance_between_points, distance_in_time};
use rand::prelude::*;
use std::collections::HashMap;

pub fn get_activity_list(route: &Route) -> Vec<String> {
    let mut activity_list: Vec<String> = Vec::with_capacity(route.allocation.len());
    for allocation_id in route.allocation.iter() {
        activity_list.push(allocation_id.clone());
    }
    activity_list
}

pub fn random_switch_route(route: &Route, activity_list: &mut Vec<String>, rand: &mut ThreadRng) {
    let number_allocations = route.allocation.len();
    let mut first_random_victim: usize = 0;
    let mut second_random_victim: usize = 0;
    while first_random_victim == second_random_victim {
        first_random_victim = rand.gen_range(0, number_allocations);
        second_random_victim = rand.gen_range(0, number_allocations);
    }
    let saved = activity_list[first_random_victim].clone();
    activity_list[first_random_victim] = activity_list[second_random_victim].clone();
    activity_list[second_random_victim] = saved;
}

pub fn build_new_route(sd: &SchemaData, route: &Route, activity_list: &Vec<String>) -> (Route, HashMap<String, Allocation>) {
    let mut new_route = Route::new(route.resource_id.clone(), route.time.clone());
    let resource = &sd.resource[&route.resource_id];
    let mut lat = resource.lat;
    let mut lon = resource.lon;
    let mut allocation: HashMap<String, Allocation> = HashMap::with_capacity(route.allocation.len());
    for id in activity_list {
        let activity = &sd.activity[id];
        let travel = distance_between_points(lat, lon, activity.lat, activity.lon);
        let travel_time = distance_in_time(travel);
        allocation.insert(id.clone(), Allocation::new(id.clone(), travel, route.time, travel_time, 0.0));
        new_route.allocation.push(id.clone());

        // Update position to this activity
        lat = activity.lat;
        lon = activity.lon;
    }
    (new_route, allocation)
}
