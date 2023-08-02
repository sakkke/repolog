mod dirs;
mod git;

use std::io;

fn main() -> io::Result<()> {
    let git_dirs = dirs::get_git_dirs(".")?;
    let mut lines: Vec<String> = git_dirs.iter()
        .filter_map(|dir| git::get_git_log(dir.path().to_str()?))
        .flatten()
        .collect();

    lines.sort();
    lines.reverse();
    println!("{}", lines.join("\n"));

    Ok(())
}
