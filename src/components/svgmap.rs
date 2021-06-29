use crate::data::schema::SchemaData;
use crate::engine::constants::*;
use crate::map::UK_POLYGONS;
use stdweb::unstable::TryFrom;
use stdweb::web::{Element, IElement, INode, Node};
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;

pub struct SVGMap {
    console: ConsoleService,
    background: String,
    data: SchemaData,
    zoom: bool,
    day: usize,

    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    lat_range: f64,
    lon_range: f64,
}

pub enum Msg {}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub background: String,
    pub data: SchemaData,
    pub zoom: bool,
    pub day: usize,
}

impl Component for SVGMap {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut obj = SVGMap {
            console: ConsoleService::new(),
            background: props.background,
            data: props.data,
            zoom: props.zoom,
            day: props.day,
            min_lat: std::f64::MAX,
            max_lat: std::f64::MIN,
            min_lon: std::f64::MAX,
            max_lon: std::f64::MIN,
            lat_range: 0.0,
            lon_range: 0.0,
        };

        // Work out extents
        obj.min_lat = std::f64::MAX;
        obj.max_lat = std::f64::MIN;
        obj.min_lon = std::f64::MAX;
        obj.max_lon = std::f64::MIN;
        for point in UK_POLYGONS.iter() {
            if point.latitude == 0.0 && point.longitude == 0.0 {
            } else {
                obj.min_lat = obj.min_lat.min(point.latitude);
                obj.max_lat = obj.max_lat.max(point.latitude);
                obj.min_lon = obj.min_lon.min(point.longitude);
                obj.max_lon = obj.max_lon.max(point.longitude);
            }
        }
        obj.lat_range = obj.max_lat - obj.min_lat;
        obj.lon_range = obj.max_lon - obj.min_lon;

        // Output some info and then return it
        obj.console.log(&format!("{} {} {} {} {} {}", obj.min_lat, obj.max_lat, obj.min_lon, obj.max_lon, obj.lat_range, obj.lon_range));
        obj
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.zoom = props.zoom;
        self.data = props.data;
        self.day = props.day;
        true
    }

    fn view(&self) -> Html<Self> {
        // Overall SVG element
        let svg = Element::try_from(js! {
            return document.createElementNS("http://www.w3.org/2000/svg", "svg");
        })
        .unwrap();
        if self.zoom {
            svg.set_attribute("viewBox", "58.5 66.75 9 7.9").unwrap();
        } else {
            svg.set_attribute("viewBox", "0 0 100 100").unwrap();
        }
        svg.set_attribute("preserveAspectRatio", "xMidYMid meet").unwrap();
        svg.set_attribute("style", &format!("background-color: {};", &self.background)).unwrap();
        svg.set_attribute("width", "100%").unwrap();
        svg.set_attribute("height", "100%").unwrap();

        // Draw the UK
        let mut points = String::new();
        let scale = 100.0;
        for point in UK_POLYGONS.iter() {
            if point.latitude == 0.0 && point.longitude == 0.0 {
                add_polygon(&svg, points);
                points = String::new();
            } else {
                let x = (point.longitude - self.min_lon) * point.latitude.to_radians() / self.lon_range * scale;
                let y = (point.latitude - self.min_lat) / self.lat_range * scale;
                points.push_str(&format!("{},{} ", x, 100.0 - y))
            }
        }
        add_polygon(&svg, points);

        // Draw all resources
        for (_id, resource) in self.data.resource.iter() {
            let x = (resource.lon - self.min_lon) * resource.lat.to_radians() / self.lon_range * scale;
            let y = (resource.lat - self.min_lat) / self.lat_range * scale;
            add_worker_square(&svg, &resource.id, x, 100.0 - y);
        }

        // Draw all activities
        for (id, activity) in self.data.activity.iter() {
            let x = (activity.lon - self.min_lon) * activity.lat.to_radians() / self.lon_range * scale;
            let y = (activity.lat - self.min_lat) / self.lat_range * scale;
            add_task_square(&svg, &activity.id, x, 100.0 - y, self.data.allocation.contains_key(id));
        }

        // Draw all routes
        for route in self.data.route.iter() {
            let day = ((&route.time - self.data.status.as_ref().unwrap().start_time) / DATETIME_ONE_DAY) as usize;
            if day == self.day {
                // Get resource and starting location
                let resource = &self.data.resource[&route.resource_id];
                let mut x1 = (resource.lon - self.min_lon) * resource.lat.to_radians() / self.lon_range * scale;
                let mut y1 = (resource.lat - self.min_lat) / self.lat_range * scale;

                // Now loop through the activities in route
                for allocation_id in &route.allocation {
                    let allocation = &self.data.allocation[allocation_id];
                    let activity = &self.data.activity[&allocation.activity_id];
                    let x2 = (activity.lon - self.min_lon) * activity.lat.to_radians() / self.lon_range * scale;
                    let y2 = (activity.lat - self.min_lat) / self.lat_range * scale;
                    add_route_line(&svg, x1, 100.0 - y1, x2, 100.0 - y2);

                    // Now move start of line to this location
                    x1 = x2;
                    y1 = y2;
                }

                // Now draw line home
                add_route_home_line(
                    &svg,
                    x1,
                    100.0 - y1,
                    (resource.lon - self.min_lon) * resource.lat.to_radians() / self.lon_range * scale,
                    100.0 - ((resource.lat - self.min_lat) / self.lat_range * scale),
                );
            }
        }

        html! {
            {VNode::VRef(Node::from(svg))}
        }
    }
}

fn add_polygon(svg: &Element, points: String) {
    let polygon = Element::try_from(js! {
        return document.createElementNS("http://www.w3.org/2000/svg", "polygon");
    })
    .unwrap();
    polygon.set_attribute("points", &points).unwrap();
    polygon.set_attribute("stroke", "#AAA").unwrap();
    polygon.set_attribute("stroke-width", "0.1").unwrap();
    polygon.set_attribute("fill", "#444").unwrap();
    svg.append_child(&polygon);
}

fn add_worker_square(svg: &Element, _id: &str, x: f64, y: f64) {
    let square = Element::try_from(js! {
        return document.createElementNS("http://www.w3.org/2000/svg", "rect");
    })
    .unwrap();
    square.set_attribute("x", &x.to_string()).unwrap();
    square.set_attribute("y", &y.to_string()).unwrap();
    square.set_attribute("style", "fill: red; stroke: black; stroke-width: 0.01;").unwrap();
    square.set_attribute("width", "0.1").unwrap();
    square.set_attribute("height", "0.1").unwrap();
    svg.append_child(&square);
}

fn add_task_square(svg: &Element, _id: &str, x: f64, y: f64, allocated: bool) {
    let square = Element::try_from(js! {
        return document.createElementNS("http://www.w3.org/2000/svg", "rect");
    })
    .unwrap();
    square.set_attribute("x", &x.to_string()).unwrap();
    square.set_attribute("y", &y.to_string()).unwrap();
    if allocated {
        square.set_attribute("style", "fill: green;").unwrap();
    } else {
        square.set_attribute("style", "fill: darkred;").unwrap();
    }
    square.set_attribute("width", "0.05").unwrap();
    square.set_attribute("height", "0.05").unwrap();
    svg.append_child(&square);
}

fn add_route_line(svg: &Element, x1: f64, y1: f64, x2: f64, y2: f64) {
    let line = Element::try_from(js! {
        return document.createElementNS("http://www.w3.org/2000/svg", "line");
    })
    .unwrap();
    line.set_attribute("x1", &x1.to_string()).unwrap();
    line.set_attribute("y1", &y1.to_string()).unwrap();
    line.set_attribute("x2", &x2.to_string()).unwrap();
    line.set_attribute("y2", &y2.to_string()).unwrap();
    line.set_attribute("style", "stroke: yellow; stroke-width: 0.02;").unwrap();
    svg.append_child(&line);
}

fn add_route_home_line(svg: &Element, x1: f64, y1: f64, x2: f64, y2: f64) {
    let line = Element::try_from(js! {
        return document.createElementNS("http://www.w3.org/2000/svg", "line");
    })
    .unwrap();
    line.set_attribute("x1", &x1.to_string()).unwrap();
    line.set_attribute("y1", &y1.to_string()).unwrap();
    line.set_attribute("x2", &x2.to_string()).unwrap();
    line.set_attribute("y2", &y2.to_string()).unwrap();
    line.set_attribute("style", "stroke: blue; stroke-width: 0.02;").unwrap();
    svg.append_child(&line);
}
