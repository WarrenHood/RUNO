use super::components::*;
use super::PlayerCount;
use crate::cards;
use crate::player;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn goto_phase(next_state: GameState) -> impl Fn(ResMut<NextState<GameState>>) {
    move |mut next_game_state: ResMut<NextState<GameState>>| {
        println!("Going to {:?} phase", next_state);
        next_game_state.set(next_state);
    }
}

pub fn spawn_players(mut commands: Commands, players: Res<PlayerCount>) {
    // I guess we can just spawn in one player for now
    let num_players = players.0;

    for i in 0..num_players {
        commands.spawn((
            player::Player,
            player::PlayerID(i),
            Name::new(format!("Player {}", i)),
        ));
        println!("Spawned player {}", i);
    }
}

pub fn despawn_players(mut commands: Commands, players_query: Query<Entity, With<player::Player>>) {
    println!("Despawning players");
    for player_entity in players_query.iter() {
        commands.entity(player_entity).despawn();
    }
}

pub fn spawn_deck(mut commands: Commands) {
    println!("Spawning deck");
    commands.spawn((Deck, Name::new("Deck")));
}

pub fn despawn_deck(mut commands: Commands, deck_query: Query<Entity, With<Deck>>) {
    println!("Despawning deck");
    for deck_entity in deck_query.iter() {
        commands.entity(deck_entity).despawn();
    }
}

pub fn despawn_cards(mut commands: Commands, cards_query: Query<Entity, With<cards::Card>>) {
    println!("Despawning all cards");
    for card_entity in cards_query.iter() {
        commands.entity(card_entity).despawn();
    }
}

pub fn spawn_cards(mut commands: Commands, deck_query: Query<Entity, With<Deck>>) {
    println!("Spawning cards into deck");
    // We will only ever try to spawn cards after spawning in the deck
    let deck = deck_query.single();
    // Add color cards
    for pack in 0..2 {
        for color in cards::COLORS.iter() {
            for rank in 0..=9 {
                if rank == 0 && pack == 1 {
                    continue;
                }
                commands
                    .spawn((cards::ColorCardNumberBundle::new(*color, rank),))
                    .set_parent(deck);
            }
            // Add action cards
            // Skip
            commands
                .spawn((
                    Name::new(format!("{:?} Skip", color)),
                    cards::Skip(1),
                    cards::DelayDraw,
                    *color,
                ))
                .set_parent(deck);
            // Draw2
            commands
                .spawn((
                    Name::new(format!("{:?} Draw 2", color)),
                    cards::Draw(2),
                    cards::DelayDraw,
                    *color,
                ))
                .set_parent(deck);
            // Reverse
            commands
                .spawn((
                    Name::new(format!("{:?} Reverse", color)),
                    cards::Reverse,
                    cards::DelayDraw,
                    *color,
                ))
                .set_parent(deck);
        }
    }

    // Add 4 wilds and wild+draw4s
    for _ in 0..4 {
        commands.spawn(cards::WildBundle::new()).set_parent(deck);
        commands
            .spawn((
                cards::WildBundle {
                    name: Name::new("Wild Draw 4"),
                    delay_draw: cards::DelayDraw,
                    wild: cards::Wild,
                    card: cards::Card,
                },
                cards::Draw(4),
            ))
            .set_parent(deck);
    }
}

pub fn deal_cards(
    mut commands: Commands,
    cards: Query<Entity, With<cards::Card>>,
    players: Query<(Entity, &player::PlayerID), With<player::Player>>,
    deck: Query<&Children, With<Deck>>,
) {
    let deck = deck.single();
    let mut rng = thread_rng();
    let mut available_cards: Vec<Entity> = deck
        .iter()
        .filter(|child| cards.contains(**child))
        .map(|child| cards.get(*child).unwrap())
        .collect();
    available_cards.shuffle(&mut rng);
    for (player_entity, player::PlayerID(player_id)) in players.iter() {
        println!("Dealing cards to player {}", player_id);
        for _ in 0..7 {
            let card = available_cards.pop();
            if let Some(card) = card {
                commands.entity(card).set_parent(player_entity);
            }
        }
    }
}

pub fn main_loop() {
    // println!("In main loop...");
}
