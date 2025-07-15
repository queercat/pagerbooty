#[macro_use]
extern crate rocket;

use crate::types::pagerduty_webhook_request::{Event, EventType, PagerdutyWebhookRequest, Priority};
use rocket::serde::json::Json;
use rocket::State;
use crate::types::buttplug::Buttplug;

mod types;

#[derive(Responder)]
enum Response {
    #[response(status = 200, content_type = "json")]
    Okay(String),
    #[response(status = 500, content_type = "json")]
    Error(String),
}

fn serialize(message: &'static str) -> String {
    format!("\"{message}\"")
}


/// I wanted this to have a smooth curve from the vibration intensity of e.g. P4 -> P1, but the API request doesn't send the previous data.
/// We could try caching and checking if we're updating something we knew previously, but it's more important to be correct.
fn handle_priority_updated(request: &Event) {
    let priority = Priority::from(request);
}

fn handle_triggered(request: &Event) {
    let priority = Priority::from(request);
}

fn handle_escalated(request: &Event) {
    let priority = Priority::from(request);
}

#[post("/", data = "<request>")]
fn handle_event(buttplug: &State<Buttplug>, request: Json<PagerdutyWebhookRequest>) -> Response {
    let event = &request.event;

    match EventType::from(&event.event_type) {
        EventType::PriorityUpdated => {
            handle_priority_updated(event);
        }
        EventType::Triggered => {
            handle_triggered(event);
        }
        EventType::Escalated => {
            handle_escalated(event);
        }
        EventType::Unimplemented => {
            dbg!("Found unimplemented incident type", &event.event_type);
        }
    };

    Response::Okay(serialize("yay"))
}

#[launch]
async fn rocket() -> _ {
    let buttplug = Buttplug::new().await;

    rocket::build().manage(buttplug).mount("/", routes![handle_event])
}
