use std::path::PathBuf;
use std::sync::LazyLock;

pub static INPUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .canonicalize()
        .expect("canonicalizing must not fail")
});
