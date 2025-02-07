use dioxus::prelude::*;

use crate::model::Mode;

pub fn use_mode_provider() {
    let mut mode_signal = Signal::new(Mode::default());

    #[cfg(not(target_os = "android"))]
    spawn(async move {
        use dioxus::desktop::tao::window::Theme;
        use midoku_theme::prelude::*;

        let window = dioxus::desktop::window();

        let mut stream = midoku_theme::subscribe().await;
        while let Some(mode) = stream.next().await {
            mode_signal.set(mode.into());
            match mode {
                Mode::Dark => window.set_theme(Some(Theme::Dark)),
                Mode::Light => window.set_theme(Some(Theme::Light)),
                Mode::Unspecified => window.set_theme(None),
            }
        }
    });

    use_context_provider(|| mode_signal);
}

pub fn use_mode() -> Signal<Mode> {
    use_context::<Signal<Mode>>()
}
