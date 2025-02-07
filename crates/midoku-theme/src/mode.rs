/// Represents the theme modes available in the system.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Dark theme mode.
    Dark,
    /// Light theme mode.
    Light,
    /// Used when the theme mode is not set or determined.
    Unspecified,
}
