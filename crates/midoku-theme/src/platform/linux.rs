use std::pin::Pin;
use std::task::{Context, Poll};

use ashpd::desktop::settings::{ColorScheme, Settings};
use tokio_stream::{empty, once, Empty, Stream, StreamExt};

use crate::mode::Mode;

impl From<ColorScheme> for Mode {
    fn from(value: ColorScheme) -> Self {
        match value {
            ColorScheme::PreferDark => Mode::Dark,
            ColorScheme::PreferLight => Mode::Light,
            ColorScheme::NoPreference => Mode::Unspecified,
        }
    }
}

enum SubscriptionStream<S> {
    Empty(Empty<Mode>),
    Some(S),
}

impl<S> SubscriptionStream<S> {
    fn empty() -> Self {
        SubscriptionStream::Empty(empty())
    }
}

impl<S> Stream for SubscriptionStream<S>
where
    S: Stream<Item = Mode> + Unpin,
{
    type Item = Mode;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut() {
            SubscriptionStream::Empty(empty) => Pin::new(empty).poll_next(cx),
            SubscriptionStream::Some(stream) => Pin::new(stream).poll_next(cx),
        }
    }
}

/// Subscribes to theme mode changes and provides a stream of `Mode` items.
///
/// This function asynchronously retrieves the current settings and subscribes
/// to changes in the color scheme. It returns a stream that initially emits the
/// current theme mode and continues to emit changes as they occur.
///
/// # Returns
///
/// An implementation of `Stream` that yields `Mode` items.
///
/// If any error occurs during setup, an empty stream is returned.
///
/// # Example
///
/// ```
/// use midoku_theme::prelude::*;
///
/// async fn example() {
///     let mut mode_stream = subscribe().await;
///
///     while let Some(mode) = mode_stream.next().await {
///         match mode {
///             Mode::Dark => println!("Dark mode activated"),
///             Mode::Light => println!("Light mode activated"),
///             Mode::Unspecified => println!("Mode is unspecified"),
///         }
///     }
/// }
/// ```
pub async fn subscribe() -> impl Stream<Item = Mode> {
    let Ok(settings) = Settings::new().await else {
        return SubscriptionStream::empty();
    };

    let Ok(initial_mode) = settings.color_scheme().await else {
        return SubscriptionStream::empty();
    };

    let Ok(stream) = settings.receive_color_scheme_changed().await else {
        return SubscriptionStream::empty();
    };

    let stream = once(initial_mode).chain(stream).map(Mode::from);
    SubscriptionStream::Some(stream)
}
