#[macro_use]
extern crate rocket;

use crate::types::pagerduty_webhook_request::PagerdutyWebhookRequest;
use rocket::serde::json::Json;

mod types;

#[derive(Responder)]
enum Response {
    #[response(status = 200, content_type = "json")]
    Okay(String),
    #[response(status = 500, content_type = "json")]
    Error(String),
}

fn serialize(message: &'static str) -> String {
    format!("\"{}\"", message)
}

enum EventType {
    Triggered,
    Escalated,
    PriorityUpdated,
    Unimplemented,
}

impl From<&String> for EventType {
    fn from(event_type: &String) -> EventType {
        match event_type.as_str() {
            "incident.escalated" => EventType::Escalated,
            "incident.triggered" => EventType::Triggered,
            "incident.priority_updated" => EventType::PriorityUpdated,
            _ => EventType::Unimplemented,
        }
    }
}

#[post("/", data = "<request>")]
fn handle_event(request: Json<PagerdutyWebhookRequest>) -> Response {
    let event = &request.event;

    match EventType::from(&event.event_type) {
        EventType::IncidentPriorityUpdated => 
        EventType::Unimplemented => dbg!("Found unimplemented incident type", &event.event_type),
    };

    Response::Okay(serialize("yay"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handle_event])
}
