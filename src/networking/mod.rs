pub mod server;
pub mod client;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
/// Game messages that are sent between the client and server
pub enum GameMessage {
    /// Message sent from the server to indicate that a specific card was drawn
    DrawCard(String),
    /// Message sent from the client to indicate that a specifc card was playable
    PlayCard(String),
    /// Message sent from the server to indicate that the client should clear their hand
    ClearHand,
    /// Message sent from the server to indicate that the discard pile should be cleared
    ClearDiscardPile,
    /// Message sent from the server to indicate the possible cards to be played
    CanPlayCards(Vec<String>)
}