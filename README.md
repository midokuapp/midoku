# Midoku App

## Development

I assume you have installed Rust, Cargo.

### Install Deno

```bash
curl -fsSL https://deno.land/x/install/install.sh | sh
```

### Then install Tauri CLI

```bash
cargo install tauri-cli --version "^2.0.0" --locked
```

### Run the app

```bash
cargo tauri dev
```

### Android

See https://v2.tauri.app/start/prerequisites/#android for the prerequisites.

> [!NOTE]
> You will not need to install the `armv7-linux-androideabi` nor
> `i686-linux-android` targets as Wasmtime+Cranelift does not support arm32.
> (https://github.com/bytecodealliance/wasmtime/issues/1173)

Initialize the Android project:

```bash
cargo tauri android init --skip-targets-install
```

To develop the app for Android, run:

```bash
cargo tauri android dev
```

To build the app for Android, run:

```bash
cargo tauri android build --apk --target aarch64 x86_64
```

To sign the APK, follow the instructions at
https://v2.tauri.app/distribute/sign/android/

## License

This project is licensed under the GNU General Public License v3.0
([LICENSE](LICENSE) or https://www.gnu.org/licenses/gpl-3.0.html)

## Disclaimer

This project does not have any affiliation with the content providers available,
and this project hosts zero content.
