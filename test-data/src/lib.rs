use std::path::{Path, PathBuf};

/// Return the path to the top-level data directory for sample files.
///
/// This uses CARGO_MANIFEST_DIR which is set at *compile time* to the manifest
/// directory of this crate (so it points to repo-root/test-data when compiled
/// in the workspace).
pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data")
}

/// convenience macro for tests:
#[macro_export]
macro_rules! sample {
    ($p:expr) => {
        test_data::data_dir().join($p)
    };
}
