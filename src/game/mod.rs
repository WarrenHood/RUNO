mod components;
mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;


#[derive(Resource)]
pub struct PlayerCount(pub u8);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(
                OnEnter(GameState::AwaitingStart),
                (clear_cards, start_game).chain(),
            )
            .add_systems(
                OnEnter(GameState::Starting),
                ((add_players, populate_deck), goto_deal_phase).chain(),
            );
    }
}
