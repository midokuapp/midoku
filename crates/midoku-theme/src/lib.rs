pub mod mode;
mod platform;

pub use platform::subscribe;

pub mod prelude {
    // Re-export for convenience.
    pub use tokio_stream::{Stream, StreamExt};

    pub use crate::mode::Mode;
    pub use crate::platform::subscribe;
}
