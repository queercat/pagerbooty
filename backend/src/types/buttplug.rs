use std::sync::Arc;
use std::time::Duration;
use buttplug::client::{ButtplugClient, ButtplugClientDevice, ScalarValueCommand};
use buttplug::core::connector::{ButtplugRemoteClientConnector, ButtplugWebsocketClientTransport};
use buttplug::core::message::serializer::ButtplugClientJSONSerializer;
use rocket::tokio::sync::Semaphore;
use rocket::tokio::task::JoinSet;
use rocket::tokio::time::{sleep};
use crate::types::pagerduty_webhook_request::Priority;

pub struct Buttplug {
    pub devices: Vec<Arc<ButtplugClientDevice>>,
    pub semaphore: Arc<Semaphore>,
}

impl Buttplug {
    pub async fn vibrate_from_priority(&self, priority: Priority) {
        let permit = self.semaphore.try_acquire();

        if permit.is_err() { return; }

        let intensity = match priority {
            Priority::P1 => 1f64,
            Priority::P2 => 0.8f64,
            Priority::P3 => 0.6f64,
            Priority::P4 => 0.4f64,
            Priority::P5 => 0.3f64,
            Priority::Unknown => 1f64,
        };

        let mut tasks = JoinSet::new();

        let mut devices = self.devices.clone();

        for device in devices {
            tasks.spawn(async move {
                device.vibrate(&ScalarValueCommand::ScalarValue(intensity)).await
            });
        }

        tasks.join_all().await;

        sleep(Duration::from_secs((4f64 * intensity) as u64)).await;

        devices = self.devices.clone();

        for device in devices {
            device.stop().await.ok();
        }
    }
}

impl Buttplug {
    pub async fn new() -> Self {
        let connector = ButtplugRemoteClientConnector::<
            ButtplugWebsocketClientTransport,
            ButtplugClientJSONSerializer,
        >::new(ButtplugWebsocketClientTransport::new_insecure_connector(
            "ws://127.0.0.1:12345",
        ));

        let client = ButtplugClient::new("Pagerbooty");

        client
            .connect(connector)
            .await
            .expect("Can't connect to Buttplug Server, exiting!");

        client.start_scanning().await.expect("Couldn't scan for devices!");

        let devices = client.devices();

        Buttplug {
            devices,
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }
}