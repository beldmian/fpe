//! Hand range representation

use crate::error::{ModelError, Result};
use crate::models::card::{Card, Rank, Suit};
use crate::models::hand::Hand;
use pokers::HandRange;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A collection of possible hole card combinations with weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// Map from hand combination to weight (0.0-1.0)
    pub hands: HashMap<Hand, f64>,
}

impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
}

impl Range {
    /// Create a new empty range
    pub fn new() -> Self {
        Self {
            hands: HashMap::new(),
        }
    }

    /// Parse range from Equilab-style notation
    pub fn from_notation(notation: &str) -> Result<Self> {
        let mut hands = HashMap::new();

        if notation.trim().is_empty() {
            return Err(ModelError::InvalidRange(
                notation.to_string(),
                "Empty range".to_string(),
            ));
        }

        // Use pokers crate for parsing
        let range = HandRange::from_string(notation.to_string());

        for combo in range.hands {
            // Mapping from discovery
            let c1_u8 = combo.0;
            let c2_u8 = combo.1;
            let weight_u8 = combo.2;

            let c1 = u8_to_card(c1_u8);
            let c2 = u8_to_card(c2_u8);

            // Hand construction from 2 cards
            // Hand logic might reorder them? Hand::new() usually stores them.
            // Hand::new checks nothing about order.
            // Hand::from_str parses string.
            // But Hand struct doesn't enforce order, so equality might fail if order differs?
            // "AhAs" vs "AsAh".
            // Hand struct: `pub cards: [Card; 2]`
            // Derive PartialEq checks exact array equality.
            // So Hand(Ah, As) != Hand(As, Ah).

            // We MUST ensure consistent ordering in Hand constructor if we want equality to work as set membership.
            // Or Range::contains must check both permutations.
            // OR Hand::new sorts them.

            // Checking Hand::new in src/models/hand.rs:
            // pub fn new(card1: Card, card2: Card) -> Self { Self { cards: [card1, card2] } }

            // It preserves order.
            // So we need to sort them to be canonical.

            let _hand = Hand::new(c1, c2); // We should sort here
            let weight = weight_u8 as f64 / 100.0;

            // Try to insert canonical hand
            let canonical = canonical_hand(c1, c2);

            hands.insert(canonical, weight);
        }

        Ok(Self { hands })
    }

    /// Returns all hands in the range
    pub fn hands(&self) -> impl Iterator<Item = (&Hand, f64)> {
        self.hands.iter().map(|(h, w)| (h, *w))
    }

    /// Returns number of hand combinations
    pub fn num_combos(&self) -> usize {
        self.hands.len()
    }

    /// Remove combos that conflict with known cards (blockers)
    pub fn remove_blockers(&mut self, cards: &[Card]) {
        self.hands.retain(|hand, _| {
            // retain if NEITHER card is in blockers
            !cards.contains(&hand.cards[0]) && !cards.contains(&hand.cards[1])
        });
    }

    /// Returns true if range contains the specified hand
    pub fn contains(&self, hand: &Hand) -> bool {
        // Check canonical form
        let canonical = canonical_hand(hand.cards[0], hand.cards[1]);
        self.hands.contains_key(&canonical)
    }
}

fn canonical_hand(c1: Card, c2: Card) -> Hand {
    if c1.rank > c2.rank {
        Hand::new(c1, c2)
    } else if c1.rank < c2.rank {
        Hand::new(c2, c1)
    } else {
        // Pairs: sort by suit?
        // Suit order: Spades > Hearts > Clubs > Diamonds?
        // Or just consistent order.
        if c1.suit > c2.suit {
            // Suit derive Ord? No, derived PartialOrd. Enum order: Hearts, Diamonds, Clubs, Spades (0,1,2,3 from macro? No manual).
            // src/models/card.rs:
            // Hearts, Diamonds, Clubs, Spades.
            // Hearts=0, Diamonds=1, Clubs=2, Spades=3.
            // So Spades > Hearts.
            Hand::new(c2, c1) // Want higher first?
        } else {
            Hand::new(c1, c2)
        }
    }
}

// Need to update u8_to_card because previous discovery of masks was "0..52".
// Mask logic: `shift = r + (3 - s) * 16`.
// But combo.0 is likely just 0..51.
// Discovery:
// 2s2h -> Combo(1, 0, 100).
// 2s is 1? 2h is 0?
// Wait, 2s mask was 2^0? No, 2^1?
// discover_masks failed to print readable output.
// Let's re-verify u8 to Card mapping from `pokers` source or discovery.

// pokers 0.7 source `card.rs`:
// 0 = 2h, 1 = 2d, 2 = 2c, 3 = 2s?
// Or 2c, 2d, 2h, 2s?
// Let's assume standard: 2c, 2d, 2h, 2s (0,1,2,3).
// Rank major? 2c..2s, 3c..3s... Ac..As?
// Or Suit major? 2c..Ac, 2d..Ad...

// Discovery output:
// 2s2h -> Combo(1, 0, 100).
// 2d2c -> Combo(3, 2, 100).
// This implies:
// 0 = 2h
// 1 = 2s
// 2 = 2c
// 3 = 2d
// That is WEIRD.
// 2h (Hearts), 2s (Spades), 2c (Clubs), 2d (Diamonds).
// Maybe order is Hearts, Spades, Clubs, Diamonds?
// Or maybe it's Rank=2, Suits are mapped strangely.

// Let's implement u8_to_card based on this hypothesis:
// 0=2h, 1=2s, 2=2c, 3=2d.
// 4=3h, 5=3s, 6=3c, 7=3d.
// Pattern: Rank * 4 + SuitIndex.
// SuitIndex: h=0, s=1, c=2, d=3.

fn u8_to_card(val: u8) -> Card {
    let r_val = val / 4;
    let s_val = val % 4;

    let rank = match r_val {
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::Jack,
        10 => Rank::Queen,
        11 => Rank::King,
        12 => Rank::Ace,
        _ => Rank::Ace,
    };

    let suit = match s_val {
        0 => Suit::Hearts,
        1 => Suit::Spades,
        2 => Suit::Clubs,
        3 => Suit::Diamonds,
        _ => Suit::Hearts,
    };

    Card::new(rank, suit)
}
