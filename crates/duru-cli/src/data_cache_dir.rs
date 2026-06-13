use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub static DATA_CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data-cache")
        .canonicalize()
        .expect("canonicalizing must not fail")
});

pub fn data_cache_file(filename: impl AsRef<Path>) -> PathBuf {
    DATA_CACHE_DIR.join(filename)
}
