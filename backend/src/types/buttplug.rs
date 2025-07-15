use buttplug::client::ButtplugClient;
use buttplug::core::connector::{ButtplugRemoteClientConnector, ButtplugWebsocketClientTransport};
use buttplug::core::message::serializer::ButtplugClientJSONSerializer;

pub struct Buttplug {
    pub client: ButtplugClient
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

        let server_name = client.server_name().expect("Can't get server name");

        let devices = client.devices();

        Buttplug {
            client
        }
    }
}