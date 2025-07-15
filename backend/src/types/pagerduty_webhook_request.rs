use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PagerdutyWebhookRequest {
    pub event: Event,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: String,
    pub resource_type: String,
    pub occurred_at: String,
    pub agent: Agent,
    pub client: Option<serde_json::Value>,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    #[serde(rename = "type")]
    pub agent_type: String,
    #[serde(rename = "self")]
    pub agent_self: String,
    pub html_url: String,
    pub summary: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "self")]
    pub data_self: String,
    pub html_url: String,
    pub number: i64,
    pub status: String,
    pub incident_key: String,
    pub created_at: String,
    pub title: String,
    pub service: Agent,
    pub assignees: Vec<Agent>,
    pub escalation_policy: Agent,
    pub teams: Vec<Option<serde_json::Value>>,
    pub priority: Agent,
    pub urgency: String,
    pub conference_bridge: Option<serde_json::Value>,
    pub resolve_reason: Option<serde_json::Value>,
    pub incident_type: IncidentType,
}

#[derive(Serialize, Deserialize)]
pub struct IncidentType {
    pub name: String,
}

#[derive(Debug)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
    P5,
    Unknown,
}

impl From<&String> for Priority {
    fn from(priority: &String) -> Priority {
        match priority.to_uppercase().as_str() {
            "P1" => Priority::P1,
            "P2" => Priority::P2,
            "P3" => Priority::P3,
            "P4" => Priority::P4,
            "P5" => Priority::P5,
            _ => Priority::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum EventType {
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

impl From<&Event> for Priority {
    fn from(event: &Event) -> Priority {
        Priority::from(&event.data.priority.summary)
    }
}
