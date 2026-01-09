//! Hand evaluator wrapper using pokers crate

use crate::models::card::{Card, Rank, Suit};
use crate::models::hand::Hand;
use pokers::Hand as PHand;

/// Evaluate hand strength (LOWER IS BETTER)
pub fn evaluate_hand(hand: &Hand, board: &[Card]) -> u64 {
    let mut mask = 0u64;

    // Add hole cards
    mask |= get_card_mask(&hand.cards[0]);
    mask |= get_card_mask(&hand.cards[1]);

    // Add board cards
    for card in board {
        mask |= get_card_mask(card);
    }

    // Create hand from mask
    let p_hand = PHand::from_bit_mask(mask);

    // Evaluate (Lower is better in pokers 0.7?)
    // In previous test: SF Score: 32925, Quads Score: 10829
    // SF > Quads. 32925 > 10829. So Higher is Better.
    // Wait, earlier output: "SF Score: 32925, Quads Score: 10829".
    // 32925 is roughly 32k.
    // Standard poker hand rank: 1 = Royal Flush? No, 1 = 7-5-4-3-2?
    // Usually 1 is best or 7462 is best.

    // Let's re-read the failing test output from previous turn (attempt 1 of this phase).
    // test_evaluation_order: QQ (4897) should beat AK (5285).
    // QQ is pair. AK is high card.
    // Pair > High Card.
    // If higher is better, 4897 < 5285, so AK > QQ. This is WRONG.
    // If lower is better, 4897 < 5285, so QQ > AK. This matches "Lower is Better".

    // BUT test_board_strength: Flush should beat AA.
    // Flush (32925) vs Quads (10829) in previous discovery.
    // Flush < Quads.
    // If Lower is Better, Quads > Flush (10k < 32k). Correct.

    // So "Lower is Better" seems consistent with:
    // Quads (10k) > Flush (32k) -> 10k < 32k. Correct.
    // Pair (4897) > High Card (5285) -> 4897 < 5285. Correct.

    // Conclusion: pokers crate uses "Lower is Better" (1 = best?).
    // We need to invert the score for our internal "Higher is Better" logic if we want to keep `calculate_equity` simple (hero > villain).
    // Or update `calculate_equity` to verify smaller is better.

    // Let's invert it here so the rest of the system sees "Higher is Better".
    // u16::MAX - score.

    let score = p_hand.evaluate();
    (u16::MAX as u64) - (score as u64)
}

fn get_card_mask(card: &Card) -> u64 {
    let r = match card.rank {
        Rank::Two => 0,
        Rank::Three => 1,
        Rank::Four => 2,
        Rank::Five => 3,
        Rank::Six => 4,
        Rank::Seven => 5,
        Rank::Eight => 6,
        Rank::Nine => 7,
        Rank::Ten => 8,
        Rank::Jack => 9,
        Rank::Queen => 10,
        Rank::King => 11,
        Rank::Ace => 12,
    };

    let s = match card.suit {
        Suit::Spades => 0,
        Suit::Hearts => 1,
        Suit::Clubs => 2,
        Suit::Diamonds => 3,
    };

    // Formula derived from discovery: shift = rank + (3 - suit) * 16
    let shift = r + (3 - s) * 16;
    1u64 << shift
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::card::Card;
    use std::str::FromStr;

    #[test]
    fn test_evaluation_order() {
        let h1 = Hand::from_str("AhKd").unwrap(); // AK
        let h2 = Hand::from_str("QsQh").unwrap(); // QQ
        let board = vec![
            Card::from_str("2c").unwrap(),
            Card::from_str("7d").unwrap(),
            Card::from_str("4h").unwrap(),
        ];

        let s1 = evaluate_hand(&h1, &board);
        let s2 = evaluate_hand(&h2, &board);

        // We inverted score, so Higher should be Better now.
        assert!(s2 > s1, "QQ ({}) should beat AK ({})", s2, s1);
    }

    #[test]
    fn test_board_strength() {
        let h1 = Hand::from_str("AsKs").unwrap(); // Flush draw -> Flush
        let board = vec![
            Card::from_str("2s").unwrap(),
            Card::from_str("5s").unwrap(),
            Card::from_str("9s").unwrap(),
            Card::from_str("Th").unwrap(),
            Card::from_str("Jd").unwrap(),
        ];

        let s1 = evaluate_hand(&h1, &board); // Flush

        let h2 = Hand::from_str("AcAd").unwrap(); // AA -> Pair
        let s2 = evaluate_hand(&h2, &board); // Pair of Aces

        assert!(s1 > s2, "Flush ({}) should beat AA ({})", s1, s2);
    }
}
