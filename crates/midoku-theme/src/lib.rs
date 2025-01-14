pub mod mode;
mod platform;

pub use platform::subscribe;

// Re-export for convenience.
pub use async_std::stream;

pub mod prelude {
    pub use async_std::stream::StreamExt;

    pub use crate::mode::Mode;
    pub use crate::platform::subscribe;
}
