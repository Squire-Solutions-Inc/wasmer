use std::env;
use std::path::PathBuf;

pub const C_ASSET_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../lib/c-api/tests/assets"
);
pub const ASSET_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/examples");

pub const WASMER_INCLUDE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../lib/c-api");

pub const WASMER_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../target/release/wasmer"
);

#[cfg(not(windows))]
pub const LIBWASMER_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../target/release/libwasmer_c_api.a"
);
#[cfg(windows)]
pub const LIBWASMER_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../target/release/wasmer_c_api.lib"
);

/// Get the path to the `libwasmer.a` static library.
pub fn get_libwasmer_path() -> PathBuf {
    PathBuf::from(
        env::var("WASMER_TEST_LIBWASMER_PATH").unwrap_or_else(|_| LIBWASMER_PATH.to_string()),
    )
}

/// Get the path to the `wasmer` executable to be used in this test.
pub fn get_wasmer_path() -> PathBuf {
    PathBuf::from(env::var("WASMER_TEST_WASMER_PATH").unwrap_or_else(|_| WASMER_PATH.to_string()))
}
