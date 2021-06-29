use crate::data::schema::{Activity, Allocation, Route, SchemaData};
use crate::engine::constants::*;
use crate::engine::optimiser::simulated_annealing;
use crate::engine::travel::{distance_between_points, distance_in_time};
use crate::engine::value::calculate_plan_value;
use std::collections::HashMap;
use std::f64::INFINITY;
use stdweb::web::Date;
use yew::services::ConsoleService;

fn schedule_ind(act: &mut HashMap<String, Activity>, _id: &String, lat: f64, lon: f64, last_lat: f64, last_lon: f64, time: f64, _console: &mut ConsoleService) -> (Option<Allocation>, f64, f64, f64) {
    let mut lowest: f64 = INFINITY;
    let mut i = "".to_string();
    for (id, act) in act.iter() {
        let dist = distance_between_points(lat, lon, act.lat, act.lon);
        if dist < lowest {
            lowest = dist;
            i = id.clone();
        }
    }

    // Did it succeed?
    if i == "" {
        let obj = (None, 0.0, 0.0, time);
        return obj;
    }

    // Calc travel
    let travel = distance_between_points(last_lat, last_lon, act[&i].lat, act[&i].lon);
    let travel_time = distance_in_time(travel);

    // Exceeding max travel, quit
    if travel_time > MAXIMUM_TRAVEL_IN_SECONDS {
        let obj = (None, 0.0, 0.0, time);
        return obj;
    }

    // Calculate new end of activity time to use on next job
    let start_time = time + travel_time * 1000.0;
    let finish_time = start_time + ACTIVITY_DURATION * 60000.0;

    // Will this job fit? I.e. are we overrunning
    if Date::get_hours(&Date::from_time(finish_time)) < 17 {
        //        console.log(&format!("{} {} {} {} {}", travel, travel_time, time.to_string(), start_time.to_string(), finish_time.to_string()));
        let obj = (Some(Allocation::new(i.clone(), travel, start_time, travel_time, 0.0)), act[&i].lat, act[&i].lon, finish_time);
        act.remove(&i);
        obj
    } else {
        let obj = (None, 0.0, 0.0, finish_time);
        obj
    }
}

fn schedule_all_resources(sd: &mut SchemaData, console: &mut ConsoleService) {
    let status = sd.status.as_mut().unwrap();
    status.changed = true;

    // Clear all previous routes first
    sd.route = Vec::new();
    sd.allocation = HashMap::new();

    // Set start time
    status.start_time = Date::from_datetime(2020, 0, 1, 9, 0, 0, 0).get_time();

    // We clone it so we don't mess up the original schema
    let mut activity_copy = sd.activity.clone();

    // Loop through each resource in turn
    let mut no_more_routes = false;
    let mut day = 0;
    while !no_more_routes {
        for (id, res) in sd.resource.iter() {
            // Set start of shift time
            let mut time = status.start_time + (DATETIME_ONE_DAY * day as f64);

            // Create route
            let mut route = Route::new(res.id.clone(), time);

            // Loop and add jobs to route
            let mut lat = res.lat;
            let mut lon = res.lon;
            let mut done = false;
            while !done {
                let (a, new_lat, new_lon, new_time) = schedule_ind(&mut activity_copy, &id, res.lat, res.lon, lat, lon, time, console);
                match a {
                    Some(x) => {
                        route.allocation.push(x.activity_id.clone());
                        lat = new_lat;
                        lon = new_lon;
                        time = new_time;
                        sd.allocation.insert(x.activity_id.clone(), x);
                    }
                    None => done = true,
                }
            }

            // No jobs?
            if route.allocation.len() == 0 {
                no_more_routes = true;
            } else {
                // Save route
                sd.route.push(route);
            }
        }
        day += 1;
    }
}

pub fn schedule_resources(sd: &mut SchemaData) {
    let mut console = ConsoleService::new();

    // Show no changes
    sd.status.as_mut().unwrap().changed = false;

    // If new data, clear routes and reschedule
    if sd.status.as_ref().unwrap().new_data {
        console.log("New data!");
        schedule_all_resources(sd, &mut console);
        sd.status.as_mut().unwrap().new_data = false;
    }

    // Simulated annealing
    let mut rng = rand::thread_rng();
    simulated_annealing(sd, &mut rng, &mut console);

    // Sum it
    let old_value = sd.status.as_ref().unwrap().value;
    let (distance, value, travel_time) = calculate_plan_value(sd);
    sd.status.as_mut().unwrap().distance = distance;
    sd.status.as_mut().unwrap().value = value;
    sd.status.as_mut().unwrap().travel_time = travel_time;

    // Quality
    let mut rate = ((value - old_value) / old_value).abs();
    if rate < 0.0 {
        rate = 0.0
    }
    if rate > 1.0 {
        rate = 1.0
    }
    sd.status.as_mut().unwrap().quality = (1.0 - rate) * 100.0;
}
