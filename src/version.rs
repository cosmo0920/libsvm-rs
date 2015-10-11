// Utility functions for libsvm versions

use ffi;

/// Returns the libsvm version as an integer
pub fn version() -> ::libc::c_int {
    ffi::version()
}

#[test]
fn test_version() {
    println!("libsvm version: {}", version());
    // 3.12 will be printed "312"
    assert!(version() >= 312);
}
