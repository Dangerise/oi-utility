use super::*;

use std::ffi::OsStr;
use std::io::{stderr, stdout};
use std::process::{Command, Stdio};

pub fn compile(source: impl AsRef<Path>, executable: impl AsRef<Path>) -> eyre::Result<()> {
    let source = source.as_ref();
    let executable = executable.as_ref();

    let status = Command::new("g++")
        .args([source.as_os_str(), OsStr::new("-o"), executable.as_os_str()])
        .stdout(stdout())
        .stdin(Stdio::null())
        .stderr(stderr())
        .status()?;

    if !status.success() {
        eyre::bail!("{:?}", status);
    }

    Ok(())
}
