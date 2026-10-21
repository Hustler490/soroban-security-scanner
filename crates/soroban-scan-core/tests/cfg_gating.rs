//! `cfg` conditionality and the test-only default (#42).

use std::path::PathBuf;

use soroban_scan_core::discovery::DiscoveryOptions;
use soroban_scan_core::project;

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/projects/cfg-gated")
}

fn function_names(options: &DiscoveryOptions) -> Vec<String> {
    let loaded = project::load(&fixture(), options).expect("cfg-gated fixture must load");
    loaded
        .sources
        .iter()
        .flat_map(|source| source.functions.iter())
        .map(|function| function.name.clone())
        .collect()
}

#[test]
fn test_only_code_is_skipped_by_default() {
    let names = function_names(&DiscoveryOptions::default());
    assert!(names.iter().any(|name| name == "shipped"));
    assert!(!names.iter().any(|name| name == "test_only"));
}

#[test]
fn include_tests_keeps_cfg_metadata() {
    let options = DiscoveryOptions {
        exclude_tests: false,
        ..DiscoveryOptions::default()
    };
    let loaded = project::load(&fixture(), &options).expect("cfg-gated fixture must load");
    let test_only = loaded
        .sources
        .iter()
        .flat_map(|source| source.functions.iter())
        .find(|function| function.name == "test_only")
        .expect("test-only function should be present when included");

    assert_eq!(test_only.cfg, vec!["test"]);
}
