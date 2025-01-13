use midoku_macros::get_config;

const CONFIG: Config = get_config!();

struct Config {
    name: &'static str,
    version: &'static str,
    identifier: &'static str,
}

pub fn use_config() -> UseConfig {
    UseConfig(())
}

#[derive(Clone, Copy)]
pub struct UseConfig(());

impl UseConfig {
    pub fn name(&self) -> &'static str {
        CONFIG.name
    }

    pub fn version(&self) -> &'static str {
        CONFIG.version
    }

    pub fn identifier(&self) -> &'static str {
        CONFIG.identifier
    }
}
