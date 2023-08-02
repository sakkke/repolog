use std::process::Command;

pub fn get_git_log(path: &str) -> Option<Vec<String>> {
    let git_log_arg = format!("--pretty=%cI | {} | %h | %s", path);
    let output = Command::new("git")
        .arg("log")
        .arg(git_log_arg)
        .current_dir(path)
        .output()
        .ok()?;

    let output = String::from_utf8_lossy(&output.stdout);
    let lines = output.trim().lines().map(|s| s.to_string()).collect();

    Some(lines)
}
