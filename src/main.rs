use bevy::prelude::*;

mod cards;
mod game;
mod player;
mod server;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(game::PlayerCount(4)) // Just gonna hardcode this for now for testing purposes
        .add_state::<GameState>()
        .add_systems(
            OnEnter(GameState::AwaitingStart),
            (clear_cards, start_game).chain(),
        )
        .add_systems(
            OnEnter(GameState::Starting),
            ((add_players, populate_deck), goto_deal_phase).chain(),
        )
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    AwaitingStart,
    Starting,
    Dealing,
    Playing,
}

pub fn start_game(mut next_game_state: ResMut<NextState<GameState>>) {
    println!("Starting the game");
    next_game_state.set(GameState::Starting);
}

pub fn goto_deal_phase(mut next_game_state: ResMut<NextState<GameState>>) {
    println!("Going to deal phase");
    next_game_state.set(GameState::Dealing);
}

pub fn add_players(mut commands: Commands, players: Res<game::PlayerCount>) {
    // I guess we can just spawn in one player for now
    let num_players = players.0;

    for i in 0..num_players {
        commands.spawn((player::Player, player::PlayerID(i)));
        println!("Spawned player {}", i);
    }
}

pub fn clear_cards(mut commands: Commands, cards_query: Query<Entity, With<cards::Card>>) {
    println!("Despawning all cards");
    for card_entity in cards_query.iter() {
        commands.entity(card_entity).despawn();
    }
}

pub fn populate_deck(mut commands: Commands) {
    println!("Populating deck");
    // Add color cards
    for pack in 0..2 {
        for color in cards::COLORS.iter() {
            for rank in 0..9 {
                if rank == 0 && pack == 1 {
                    continue;
                }
                commands.spawn((
                    cards::InDeck,
                    cards::ColorCardNumberBundle::new(*color, rank),
                ));
            }
            // Add action cards
            // Skip
            commands.spawn((
                cards::InDeck,
                cards::CardName(format!("{:?} Skip", color)),
                cards::Skip(1),
                cards::DelayDraw,
                *color,
            ));
            // Draw2
            commands.spawn((
                cards::InDeck,
                cards::CardName(format!("{:?} Draw 2", color)),
                cards::Draw(2),
                cards::DelayDraw,
                *color,
            ));
            // Reverse
            commands.spawn((
                cards::InDeck,
                cards::CardName(format!("{:?} Reverse", color)),
                cards::Reverse,
                cards::DelayDraw,
                *color,
            ));
        }
    }

    // Add 4 wilds and wild+draw4s
    for _ in 0..4 {
        commands.spawn((cards::InDeck, cards::WildBundle::new()));
        commands.spawn((
            cards::InDeck,
            cards::WildBundle {
                name: cards::CardName("Wild Draw 4".into()),
                delay_draw: cards::DelayDraw,
                wild: cards::Wild,
                card: cards::Card,
            },
            cards::Draw(4),
        ));
    }
}
