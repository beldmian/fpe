use fpe::models::{Action, ActionStrategy, Strategy};
use serde_json::Value;

#[test]
fn test_json_structure() {
    let actions = vec![
        ActionStrategy {
            action: Action::Fold,
            frequency: 0.2,
            ev: 0.0,
        },
        ActionStrategy {
            action: Action::Call,
            frequency: 0.8,
            ev: 1.0,
        },
    ];

    let strategy = Strategy::new(actions, 5000, 0.002);

    let json_string = serde_json::to_string(&strategy).expect("Failed to serialize");
    let json: Value = serde_json::from_str(&json_string).expect("Failed to parse JSON");

    // Check top level fields
    assert!(json.get("actions").is_some());
    assert!(json.get("iterations").is_some());
    assert!(json.get("convergence").is_some());

    // Check values
    assert_eq!(json["iterations"], 5000);

    // Check actions array
    let actions_array = json["actions"].as_array().expect("actions should be array");
    assert_eq!(actions_array.len(), 2);

    // Check first action structure
    let first_action = &actions_array[0];
    assert!(first_action.get("action").is_some());
    assert!(first_action.get("frequency").is_some());
    assert!(first_action.get("ev").is_some());

    assert_eq!(first_action["action"], "Fold");
}
