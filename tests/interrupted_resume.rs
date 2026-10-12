use op15_adaptive_harness::state::DurableState;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn state_survives_round_trip() {
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    // Keep the fixture inside Cargo's target directory: Windows sandboxed runners
    // can deny access to the system-wide temporary directory.
    let path = std::env::current_dir()
        .unwrap()
        .join("target")
        .join(format!("op15-{id}.state"));
    let expected = DurableState {
        task_id: "t".into(),
        status: "running".into(),
        last_decision: "retrieve".into(),
    };
    expected.save_atomic(&path).unwrap();
    assert_eq!(DurableState::load(&path).unwrap(), expected);
    std::fs::remove_file(path).unwrap();
}
