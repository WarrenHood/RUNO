use bevy::prelude::*;

mod cards;
mod game;
mod player;
mod server;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(game::PlayerCount(4)) // Just gonna hardcode this for now for testing purposes
        .add_plugins(game::GamePlugin)
        .run();
}


