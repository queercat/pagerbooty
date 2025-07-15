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


#[post("/", data = "<request>")]
async fn handle_event(buttplug: &State<Buttplug>, request: Json<PagerdutyWebhookRequest>) -> Response {
    let event = &request.event;
    let priority = Priority::from(event);

    match EventType::from(&event.event_type) {
        EventType::PriorityUpdated => {
            buttplug.vibrate_from_priority(priority).await;
        }
        EventType::Triggered => {
            buttplug.vibrate_from_priority(priority).await;
        }
        EventType::Escalated => {
            buttplug.vibrate_from_priority(priority).await;
        }
        EventType::Unimplemented => {
            dbg!("Found unimplemented incident type", &event.event_type);
            return Response::Error("Unimplemented incident type".into());
        }
    };

    Response::Okay(serialize("Bzzzt"))
}

#[launch]
async fn rocket() -> _ {
    let buttplug = Buttplug::new().await;

    rocket::build().manage(buttplug).mount("/", routes![handle_event])
}
