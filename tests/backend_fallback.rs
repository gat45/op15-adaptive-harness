use op15_adaptive_harness::hardware::HardwareCost;
#[test]
fn complete_hardware_cost_includes_fallback() {
    let c = HardwareCost {
        allocation_mb: 100,
        transfer_ms: 4,
        dispatch_ms: 3,
        sync_ms: 2,
        fallback_ms: 9,
    };
    assert_eq!(c.end_to_end(5, 1).wall_ms, 18);
}
