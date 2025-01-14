#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::subscribe;

#[cfg(not(any(target_os = "linux")))]
mod unknown {
    use async_std::stream::{once, Stream};

    use crate::error::Result;
    use crate::Mode;

    pub async fn subscribe() -> impl Stream<Item = Mode> {
        once(Mode::Unspecified)
    }
}

#[cfg(not(any(target_os = "linux")))]
pub use unknown::subscribe;
