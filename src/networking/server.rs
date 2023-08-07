use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

use super::GameMessage;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);

        let server = RenetServer::new(ConnectionConfig::default());
        let public_addr = "0.0.0.0:5000".parse().unwrap();
        let socket = UdpSocket::bind(public_addr).unwrap();
        let server_config = ServerConfig {
            max_clients: 64,
            protocol_id: 0,
            public_addr,
            authentication: ServerAuthentication::Unsecure,
        };
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
        app.insert_resource(server);
        app.insert_resource(transport);

        // Add the client messages and message queue resources
        app.insert_resource(ClientMessages(Vec::new()));
        app.insert_resource(MessageQueue(Vec::new()));

        app.add_systems(Update, send_message_system);
        app.add_systems(Update, receive_message_system);
        app.add_systems(Update, handle_events_system);
    }
}

/// Sends all queued messages to their clients
fn send_message_system(mut server: ResMut<RenetServer>, mut message_queue: ResMut<MessageQueue>) {
    for (client_id, message) in message_queue.0.iter() {
        if client_id.is_some() {
            println!("Sending message to client {client_id:#?}: {message:#?}");
        } else {
            println!("Broadcasting message: {message:#?}");
        }
        if let Ok(message) = bincode::serialize::<GameMessage>(message) {
            if let Some(client_id) = *client_id {
                server.send_message(client_id, DefaultChannel::ReliableOrdered, message);
            } else {
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        } else {
            eprintln!("Failed to serialize message to send to client {client_id:#?}: {message:#?}");
        }
    }
    message_queue.0.clear();
}

/// Receive messages from clients and put them in the `ClientMessages` resource
fn receive_message_system(
    mut server: ResMut<RenetServer>,
    mut client_messages: ResMut<ClientMessages>,
) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            if let Ok(message) = bincode::deserialize::<GameMessage>(&message) {
                println!("Got message from client {client_id}: {message:#?}");
                client_messages.0.push((client_id, message));
            } else {
                eprintln!("Failed to decode a message from client {client_id}");
            }
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}

#[derive(Resource)]
/// A list of messages received from different clients.
/// Should be processed by some bevy system
/// Messages are tuples of (<client_id(u64)>, <GameMessage>)
pub struct ClientMessages(pub Vec<(u64, GameMessage)>);

impl ClientMessages {
    pub fn pop_message(&mut self) -> Option<(u64, GameMessage)> {
        if !self.0.is_empty() {
            self.0.pop()
        } else {
            None
        }
    }
}

#[derive(Resource)]
/// A list of messages to send to each client.
/// Add messages here to queue them for sending to the server
/// Messages are tuples of (<client_id(u64)>, <GameMessage>)
/// If the client id is `None`, the message will be broadcasted instead
pub struct MessageQueue(pub Vec<(Option<u64>, GameMessage)>);

impl MessageQueue {
    pub fn send_message(&mut self, client_id: Option<u64>, message: GameMessage) {
        self.0.push((client_id, message));
    }
}
