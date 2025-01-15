#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::subscribe;

#[cfg(not(any(target_os = "linux")))]
mod unknown {
    use tokio_stream::{once, Stream};

    use crate::mode::Mode;

    pub async fn subscribe() -> impl Stream<Item = Mode> {
        once(Mode::Unspecified)
    }
}

#[cfg(not(any(target_os = "linux")))]
pub use unknown::subscribe;
