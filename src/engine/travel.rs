use std::f64::consts::PI;

static EARTH_RADIUS_M: f64 = 6_367_450.0;
static CONVERT_RAD: f64 = PI / 180.0;
static SECONDS_PER_METRE: f64 = 0.0559234073;

pub fn distance_between_points(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let start_lat_in_rad: f64 = lat1 * CONVERT_RAD;
    let start_lon_in_rad: f64 = lon1 * CONVERT_RAD;
    let end_lat_in_rad: f64 = lat2 * CONVERT_RAD;
    let end_lon_in_rad: f64 = lon2 * CONVERT_RAD;
    let longitude: f64 = end_lon_in_rad - start_lon_in_rad;
    let latitude: f64 = end_lat_in_rad - start_lat_in_rad;
    let sin_half_lat: f64 = (latitude * 0.5).sin();
    let sin_half_lon: f64 = (longitude * 0.5).sin();
    let a: f64 = sin_half_lat * sin_half_lat + (start_lat_in_rad).cos() * (end_lat_in_rad).cos() * sin_half_lon * sin_half_lon;
    let c: f64 = a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_M * (c + c)
}

pub fn distance_in_time(distance: f64) -> f64 {
    distance * SECONDS_PER_METRE
}
