use std::process::Command;
use std::str;

#[test]
fn test_cli_error_messages() {
    // 1. Invalid Card
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--",
            "analyze",
            "--hero",
            "AhXX", // Invalid card
            "--board",
            "",
            "--villain-range",
            "AA",
            "--pot",
            "10",
            "--stack",
            "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();

    // Debug what we got
    println!("Invalid Card Error: {}", stderr);

    assert!(stderr.contains("Error parsing hero hand"));
    // The exact error message from Hand::from_str contains 'X' or 'XX' or similar.
    // The actual error was "Invalid card 'X': expected format..."
    // So checking for "Invalid card" is safer.
    assert!(stderr.contains("Invalid card"));

    // 2. Duplicate Card
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--",
            "analyze",
            "--hero",
            "AhKd",
            "--board",
            "AhTs2c", // Duplicate Ah
            "--villain-range",
            "AA",
            "--pot",
            "10",
            "--stack",
            "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    println!("Duplicate Card Error: {}", stderr);

    assert!(stderr.contains("Duplicate card"));

    // 3. Invalid Game State (Pot < 0)
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--",
            "analyze",
            "--hero",
            "AhKd",
            "--board",
            "",
            "--villain-range",
            "AA",
            "--pot",
            "-10", // Invalid pot
            "--stack",
            "100",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    println!("Invalid Pot Error: {}", stderr);

    assert!(
        stderr.contains("Error creating game state")
            || stderr.contains("Pot size must be greater than 0")
            || stderr.contains("unexpected argument")
            || stderr.contains("invalid value")
    );
}
