fn main() {
    tailwind::download_binary();
    tailwind::build_css();

    println!("cargo:rerun-if-changed=./assets/*");
    println!("cargo:rerun-if-changed=./src/**/*");
}

mod tailwind {
    use std::env;
    use std::io::Write;
    use std::path::PathBuf;

    #[cfg(target_arch = "x86_64")]
    const TARGET_ARCH: &str = "x64";
    #[cfg(target_arch = "aarch64")]
    const TARGET_ARCH: &str = "arm64";

    #[cfg(target_os = "linux")]
    const BINARY_NAME: &str = ::const_format::concatcp!("tailwindcss-linux-", TARGET_ARCH);
    #[cfg(target_os = "macos")]
    const BINARY_NAME: &str = ::const_format::concatcp!("tailwindcss-macos-", TARGET_ARCH);
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    const BINARY_NAME: &str = "tailwindcss-windows-x64.exe";

    const URL: &str = ::const_format::concatcp!(
        "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/",
        BINARY_NAME
    );

    const INPUT_CSS: &str = "./assets/input.css";
    const OUTPUT_CSS: &str = "./assets/tailwind.css";

    fn out_dir() -> PathBuf {
        let out_dir = env::var("OUT_DIR").expect("could not read `OUT_DIR` environement variable.");
        PathBuf::from(out_dir)
    }

    fn binary_file() -> PathBuf {
        out_dir().join(BINARY_NAME)
    }

    /// Downloads the Tailwind binary
    pub fn download_binary() {
        let binary_file = binary_file();
        if binary_file.exists() {
            return;
        }

        let response = ::reqwest::blocking::get(URL).expect("failed to download Tailwind binary");
        let bytes = response.bytes().unwrap();
        std::fs::write(binary_file.clone(), bytes).expect("failed to write Tailwind binary");

        #[cfg(unix)]
        std::fs::set_permissions(
            binary_file,
            std::os::unix::fs::PermissionsExt::from_mode(0o755),
        )
        .expect("failed to set executable permissions");
    }

    /// Compiles the output css from the input css
    pub fn build_css() {
        // Initialize the input css with base tailwind import
        if !PathBuf::from(INPUT_CSS).exists() {
            std::fs::write(INPUT_CSS, b"@import \"tailwindcss\";\n")
                .expect("could not initialize {INPUT_CSS}");
        }

        match std::process::Command::new(binary_file())
            .args(["-i", INPUT_CSS, "-o", OUTPUT_CSS, "--minify"])
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    let _ = std::io::stdout().write_all(&output.stdout);
                    let _ = std::io::stdout().write_all(&output.stderr);
                    panic!("Tailwind error");
                }
            }
            Err(err) => panic!("Tailwind error: {:?}", err),
        }
    }
}
