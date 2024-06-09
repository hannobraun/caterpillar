mod debounce;
mod watcher;

pub use self::watcher::Watcher;

pub fn watch() -> anyhow::Result<Watcher> {
    Watcher::new()
}
