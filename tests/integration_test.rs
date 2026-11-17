// Integration test for SigmaOS-dhcp-relay
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use dhcp_relay as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
