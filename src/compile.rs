use super::*;

use std::ffi::OsStr;
use std::io::{stderr, stdout};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn compile(source: impl AsRef<Path>, executable: impl AsRef<Path>) -> eyre::Result<()> {
    let source = source.as_ref();
    let executable = executable.as_ref();

    let status = Command::new("g++")
        .args([
            source.as_os_str(),
            OsStr::new("-o"),
            executable.as_os_str(),
            OsStr::new("-Wall"),
            OsStr::new("-Wextra"),
        ])
        .stdout(stdout())
        .stdin(Stdio::null())
        .stderr(stderr())
        .status()?;

    if !status.success() {
        eyre::bail!("{:?}", status);
    }

    Ok(())
}

pub fn compile_auto_name(path: impl AsRef<Path>) -> eyre::Result<PathBuf> {
    let path = path.as_ref();
    let directory = path.parent().unwrap();

    // dbg!(directory);

    let executable = directory.join(path.file_stem().unwrap());

    // dbg!(&executable);
    
    compile::compile(path, &executable)?;
    
    Ok(executable)
}
