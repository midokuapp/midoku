# Midoku App

## Development

I assume you have followed the [Getting Started](https://dioxuslabs.com/learn/0.6/getting_started/)
guide from Dioxus and have installed `dioxus-cli`.

You also need Tailwind's standalone CLI: https://tailwindcss.com/blog/standalone-cli.

### Run the app

```bash
dx serve
```

### Android

See https://dioxuslabs.com/learn/0.6/guides/mobile/#android for the prerequisites.

<!--
> [!NOTE]
> You will not need to install the `armv7-linux-androideabi` nor
> `i686-linux-android` targets as Wasmtime+Cranelift does not support arm32.
> (https://github.com/bytecodealliance/wasmtime/issues/1173)

Initialize the Android project:

```bash
cargo tauri android init --skip-targets-install
```
--->

To develop the app for Android, run:

```bash
dx serve --platform android
```

<!--
To build the app for Android, run:

```bash
cargo tauri android build --apk --target aarch64 x86_64
```

To sign the APK, follow the instructions at
https://v2.tauri.app/distribute/sign/android/
--->

## License

The Midoku application located in [src/](src/) is licensed under the GNU General Public License v3.0
([LICENSE](LICENSE) or https://www.gnu.org/licenses/gpl-3.0.html)

The helper crates located in [crates/](crates/) are licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This project does not have any affiliation with the content providers available,
and this project hosts zero content.
