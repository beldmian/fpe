use fpe::models::card::Card;
use fpe::models::hand::Hand;
use fpe::models::range::Range;
use std::str::FromStr;

#[test]
fn test_basic_range_parsing() {
    // Pair
    let range = Range::from_notation("AA").expect("Failed to parse AA");
    assert!(range.contains(&Hand::from_str("AhAs").unwrap()));
    assert_eq!(range.num_combos(), 6);

    // Suited
    let range = Range::from_notation("AKs").expect("Failed to parse AKs");
    assert!(range.contains(&Hand::from_str("AhKh").unwrap()));
    assert!(!range.contains(&Hand::from_str("AhKd").unwrap())); // Offsuit
    assert_eq!(range.num_combos(), 4);

    // Offsuit
    let range = Range::from_notation("AKo").expect("Failed to parse AKo");
    assert!(range.contains(&Hand::from_str("AhKd").unwrap()));
    assert!(!range.contains(&Hand::from_str("AhKh").unwrap())); // Suited
    assert_eq!(range.num_combos(), 12);
}

#[test]
fn test_plus_dash_notation() {
    // Plus
    let range = Range::from_notation("KK+").expect("Failed to parse KK+");
    assert!(range.contains(&Hand::from_str("AhAs").unwrap())); // AA
    assert!(range.contains(&Hand::from_str("KhKs").unwrap())); // KK
    assert!(!range.contains(&Hand::from_str("QhQs").unwrap())); // QQ
    assert_eq!(range.num_combos(), 12); // 6 AA + 6 KK

    // Dash (Range)
    // "JJ-99" -> JJ, TT, 99
    let range = Range::from_notation("JJ-99").expect("Failed to parse JJ-99");
    assert!(range.contains(&Hand::from_str("JhJs").unwrap()));
    assert!(range.contains(&Hand::from_str("ThTs").unwrap()));
    assert!(range.contains(&Hand::from_str("9h9s").unwrap()));
    assert!(!range.contains(&Hand::from_str("8h8s").unwrap()));
    assert_eq!(range.num_combos(), 18); // 3 * 6
}

#[test]
fn test_combined_notation() {
    let range = Range::from_notation("AA,KK").expect("Failed to parse AA,KK");
    assert_eq!(range.num_combos(), 12);
}

#[test]
fn test_blocker_removal() {
    let mut range = Range::from_notation("AA").expect("Failed to parse AA");
    // AA has 6 combos: AhAs, AhAc, AhAd, AsAc, AsAd, AcAd

    // Block Ah (Hero has Ah)
    let blockers = vec![Card::from_str("Ah").unwrap()];
    range.remove_blockers(&blockers);

    // Should remove 3 combos (AhAs, AhAc, AhAd)
    // Remaining: AsAc, AsAd, AcAd (3 combos)
    if range.num_combos() != 3 {
        println!("Expected 3 combos, got {}:", range.num_combos());
        for (h, _) in range.hands() {
            println!("  {}", h.notation());
        }
    }
    assert_eq!(range.num_combos(), 3);
    assert!(!range.contains(&Hand::from_str("AhAs").unwrap()));
    assert!(range.contains(&Hand::from_str("AsAc").unwrap()));
}
