use std::io::Write;

fn main() {
    match std::process::Command::new("tailwindcss")
        .args([
            "-i",
            "./assets/input.css",
            "-o",
            "./assets/tailwind.css",
            "--minify",
        ])
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

    println!("cargo:rerun-if-changed=./assets/*");
    println!("cargo:rerun-if-changed=./src/**/*");
}
