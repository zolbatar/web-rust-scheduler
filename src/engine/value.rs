use crate::data::schema::{Allocation, Resource, Route, SchemaData};
use crate::engine::constants::*;
use crate::engine::travel::{distance_between_points, distance_in_time};
use std::collections::HashMap;

pub fn calculate_plan_value(sd: &SchemaData) -> (f64, f64, f64) {
    let mut sum: f64 = 0.0;
    let mut value: f64 = 0.0;
    let mut travel_time: f64 = 0.0;
    for route in sd.route.iter() {
        let (new_sum, new_value, new_travel_time) = calculate_route_value(sd, &route, &sd.allocation);
        sum += new_sum;
        value += new_value;
        travel_time += new_travel_time;
    }
    (sum, value, travel_time)
}

pub fn calculate_route_value(sd: &SchemaData, route: &Route, allocations: &HashMap<String, Allocation>) -> (f64, f64, f64) {
    let mut sum: f64 = 0.0;
    let mut value: f64 = 0.0;
    let mut travel_time: f64 = 0.0;

    // Get resource and start locations
    let resource: &Resource = &sd.resource[&route.resource_id];
    let mut lat = resource.lat;
    let mut lon = resource.lon;

    // Loop through all allocations
    for allocation_id in route.allocation.iter() {
        // Get allocation
        let all = &allocations[allocation_id];

        // Get activity
        let activity = &sd.activity[allocation_id];

        sum += all.dist;
        let this_total_travel = all.travel_to + all.travel_from;
        travel_time += this_total_travel;
        value += ACTIVITY_VALUE - (ACTIVITY_DURATION * HOURLY_WAGE / 360.0) - (this_total_travel * HOURLY_WAGE / 360.0);

        // Update position to this activity
        lat = activity.lat;
        lon = activity.lon;
    }

    // We also need to consider the cost of travelling home
    let travel = distance_between_points(lat, lon, resource.lat, resource.lon);
    value -= distance_in_time(travel) * HOURLY_WAGE / 360.0;

    (sum, value, travel_time)
}
