use std::path::PathBuf;

pub fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let raw_path = std::str::from_utf8(&output).unwrap().trim();
    let path = PathBuf::from(raw_path);
    path.parent().unwrap().to_path_buf()
}
