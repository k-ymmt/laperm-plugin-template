//! Keeps `manifest.json` valid and in step with `Cargo.toml` before a release.

use laperm_plugin::manifest::Manifest;

#[test]
fn manifest_is_valid_and_matches_the_crate_version() {
    let manifest: Manifest = serde_json::from_str(include_str!("../manifest.json")).expect("manifest.json parses");
    manifest.validate().expect("manifest.json follows the rules Laperm enforces");
    assert_eq!(manifest.version, env!("CARGO_PKG_VERSION"), "manifest.json version must equal Cargo.toml version");
    assert!(manifest.commands.iter().any(|c| c.id == "insert-date"));
}
