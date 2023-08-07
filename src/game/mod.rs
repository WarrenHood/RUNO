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
                (
                    despawn_deck,
                    despawn_players,
                    despawn_cards,
                )
                    .chain(),
            )
            .add_systems(Update, wait_for_players.run_if(in_state(GameState::AwaitingStart)))
            .add_systems(
                OnEnter(GameState::Starting),
                (
                    spawn_players,
                    spawn_deck,
                    apply_deferred,
                    spawn_cards,
                    goto_phase(GameState::Dealing),
                )
                    .chain(),
            )
            .add_systems(
                OnEnter(GameState::Dealing),
                (deal_cards, goto_phase(GameState::Playing)).chain(),
            )
            .add_systems(Update, main_loop.run_if(in_state(GameState::Playing)));
    }
}
