// https://blog.sentry.io/you-cant-rust-that/

use std::sync::{Arc, RwLock};

#[derive(Default)]
struct ConfigInner {
    debug_mode: bool,
}

struct Config {
    inner: RwLock<ConfigInner>,
}

impl Config {
    pub fn new() -> Arc<Config> {
        Arc::new(Config {
            inner: RwLock::new(Default::default()),
        })
    }
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.clone())
    }
    pub fn debug_mode(&self) -> bool {
        self.inner.read().unwrap().debug_mode
    }
    pub fn set_debug_mode(&self, value: bool) {
        self.inner.write().unwrap().debug_mode = value;
    }
}

thread_local! {
    static CURRENT_CONFIG: Arc<Config> = Config::new();
}

fn main() {
    let config = Config::current();
    config.set_debug_mode(true);
    if config.debug_mode() {
        println!("in debug mode");
    }
}

//If you do not need this type to work with threads you can also replace Arc with Rc and RwLock with RefCell.
