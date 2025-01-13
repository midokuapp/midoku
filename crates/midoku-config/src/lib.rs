use midoku_macros::get_config;

pub const CONFIG: Config = get_config!();

pub struct Config {
    pub name: &'static str,
    pub version: &'static str,
    pub identifier: &'static str,
}
