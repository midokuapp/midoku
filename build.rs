fn main() {
    deno::init();
    deno::compile();

    println!("cargo:rerun-if-changed=./assets/*");
    println!("cargo:rerun-if-changed=./src/**/*");
}

mod deno {
    const INPUT_CSS: &str = "./assets/input.css";
    const OUTPUT_CSS: &str = "./assets/tailwind.css";

    /// Initialize tailwind and daisyui
    pub fn init() {
        match std::process::Command::new("deno")
            .args([
                "install",
                "--dev",
                "npm:tailwindcss",
                "npm:@tailwindcss/cli",
                "npm:daisyui@beta",
            ])
            .stdout(std::io::stdout())
            .stderr(std::io::stderr())
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    panic!("deno error");
                }
            }
            Err(err) => panic!("error attempting to run deno: {err:?}"),
        };
    }

    /// Compile CSS
    pub fn compile() {
        match std::process::Command::new("deno")
            .args([
                "run",
                "--allow-all",
                "--node-modules-dir=auto",
                "npm:@tailwindcss/cli",
                "--input",
                INPUT_CSS,
                "--output",
                OUTPUT_CSS,
                "--minify",
            ])
            .stdout(std::io::stdout())
            .stderr(std::io::stderr())
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    panic!("deno error");
                }
            }
            Err(err) => panic!("error attempting to run deno: {err:?}"),
        };
    }
}
