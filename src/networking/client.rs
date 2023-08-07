use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;

use super::GameMessage;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);

        let client = RenetClient::new(ConnectionConfig::default());
        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: 0,
            server_addr,
            user_data: None,
        };
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        app.insert_resource(client);
        app.insert_resource(transport);

        // Add message queue and server messages
        app.insert_resource(MessageQueue(Vec::new()));
        app.insert_resource(ServerMessages(Vec::new()));

        app.add_systems(Update, send_message_system);
        app.add_systems(Update, receive_message_system);
    }
}

/// Sends all queued messages to the server
fn send_message_system(mut client: ResMut<RenetClient>, mut message_queue: ResMut<MessageQueue>) {
    for message in message_queue.0.iter() {
        println!("Sending message: {message:#?} to server");
        if let Ok(message) = bincode::serialize(message) {
            client.send_message(DefaultChannel::ReliableOrdered, message);
        } else {
            eprintln!("Failed to serialize message: {message:#?}");
        }
    }
    message_queue.0.clear();
}

/// Places all received messages into the `ServerMessages` resource
fn receive_message_system(
    mut client: ResMut<RenetClient>,
    mut server_messages: ResMut<ServerMessages>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(message) = bincode::deserialize::<GameMessage>(&message) {
            println!("Got message from server: {message:#?}");
            server_messages.0.push(message);
        } else {
            eprintln!("Failed to deserialize a message from the server");
        }
    }
}

#[derive(Resource)]
/// A list of messages received from the server.
/// Should be processed by some bevy system
pub struct ServerMessages(Vec<GameMessage>);

impl ServerMessages {
    pub fn pop_message(&mut self) -> Option<GameMessage> {
        if !self.0.is_empty() {
            self.0.pop()
        } else {
            None
        }
    }
}

#[derive(Resource)]
/// A list of messages to send to the server.
/// Add messages here to queue them for sending to the server
pub struct MessageQueue(Vec<GameMessage>);
impl MessageQueue {
    pub fn send_message(&mut self, message: GameMessage) {
        self.0.push(message);
    }
}
