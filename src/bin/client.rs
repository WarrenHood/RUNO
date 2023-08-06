use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use runo::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RUNO Client".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(networking::client::ClientPlugin)
        .add_plugins(WorldInspectorPlugin::new()) // Just gonna hardcode this for now for testing purposes
        .run();
}