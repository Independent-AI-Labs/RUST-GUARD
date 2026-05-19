use std::process::Command;

#[test]
fn guard_binary_exists() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "--version"])
        .output()
        .expect("failed to execute cargo run");
    assert!(output.status.success(), "guard binary should run");
}

#[test]
fn guard_blocks_reset() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "reset", "--hard"])
        .output()
        .expect("failed to execute cargo run");
    assert!(!output.status.success(), "guard should block git reset");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("BLOCKED") || stderr.contains("blocked"),
        "stderr should mention blocked: {stderr}"
    );
}
