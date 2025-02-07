#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dark,
    #[default]
    Light,
}

impl From<midoku_theme::prelude::Mode> for Mode {
    fn from(value: midoku_theme::prelude::Mode) -> Self {
        match value {
            midoku_theme::prelude::Mode::Dark => Mode::Dark,
            midoku_theme::prelude::Mode::Light => Mode::Light,
            midoku_theme::prelude::Mode::Unspecified => Mode::default(),
        }
    }
}
