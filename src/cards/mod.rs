use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue
}

pub static COLORS: [Color; 4] = [Color::Red, Color::Yellow, Color::Green, Color::Blue];

#[derive(Component)]
pub struct Rank(pub u8);

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct DelayDraw;

#[derive(Component)]
pub struct Draw(pub u8);

#[derive(Component)]
pub struct Wild;

#[derive(Component)]
pub struct Reverse;

#[derive(Component)]
pub struct Skip(pub u8);

#[derive(Component)]
pub struct CardName(pub String);

#[derive(Component)]
pub struct InDeck;

#[derive(Bundle)]
pub struct ColorCardNumberBundle {
    pub name: CardName,
    pub color: Color,
    pub rank: Rank,
    pub card: Card
}

impl ColorCardNumberBundle {
    pub fn new(color: Color, rank: u8) -> Self {
        Self { name: CardName(format!("{:?} {}", color, rank)), color, rank: Rank(rank), card: Card }
    }
}

#[derive(Bundle)]
pub struct WildBundle {
    pub name: CardName,
    pub delay_draw: DelayDraw,
    pub wild: Wild,
    pub card: Card
}

impl WildBundle {
    pub fn new() -> Self {
        Self { name: CardName("Wild".into()), delay_draw: DelayDraw, wild: Wild, card: Card }
    }
}