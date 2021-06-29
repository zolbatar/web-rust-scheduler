use crate::data::schema::{Route, SchemaData};
use crate::engine::constants::*;
use crate::engine::route::*;
use crate::engine::value::calculate_route_value;
use rand::prelude::*;
use yew::services::ConsoleService;

pub fn simulated_annealing(sd: &mut SchemaData, rand: &mut ThreadRng, _console: &mut ConsoleService) {
    // New route list
    let mut new_route_list: Vec<Route> = Vec::new();

    for route in sd.route.iter() {
        // Do we have at least 2 allocations? if not, this is all a bit pointless
        if route.allocation.len() > 1 {
            let mut route_temp = route.clone();
            for _ in 0..SIMULATED_ANNEALING_ITERATIONS {
                // Calculate the baseline to compare against
                let mut activity_list = get_activity_list(&route);
                let (_, original_value, _) = calculate_route_value(sd, &route, &sd.allocation);

                // Now we should try switching things around
                random_switch_route(&route, &mut activity_list, rand);

                // Now we have re-sorted things, let's build the new route
                let (new_route, new_allocations) = build_new_route(&sd, &route, &activity_list);

                // Now work out value
                let (_, new_value, _) = calculate_route_value(sd, &new_route, &new_allocations);
                if new_value > original_value {
                    route_temp = new_route;
                    for (key, value) in new_allocations {
                        sd.allocation.insert(key, value);
                    }
                    sd.status.as_mut().unwrap().changed = true;
                }
            }
            new_route_list.push(route_temp);
        } else {
            new_route_list.push(route.clone());
        }
    }

    // Replace schema route list
    sd.route = new_route_list;
}

//pub fn consider_removing() {}
