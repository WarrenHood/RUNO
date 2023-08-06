use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use runo::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RUNO Server".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(networking::server::ServerPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(game::PlayerCount(4)) // Just gonna hardcode this for now for testing purposes
        .add_plugins(game::GamePlugin)
        .run();
}


