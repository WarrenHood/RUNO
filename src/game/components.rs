use bevy::prelude::*;

#[derive(Component)]
pub struct Deck;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    AwaitingStart,
    Starting,
    Dealing,
    Playing,
}