use eyre::{ensure, Context};

use super::*;

use std::ffi::OsStr;
use std::io::{stderr, stdout};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn compile(source: impl AsRef<Path>, executable: impl AsRef<Path>) -> eyre::Result<()> {
    let source = source.as_ref();
    let executable = executable.as_ref();

    ensure!(
        source.exists(),
        "Source code : \"{}\" do not exist !",
        source.display()
    );

    let mut args = vec![source.as_os_str(), OsStr::new("-o"), executable.as_os_str()];
    for flag in include_str!("gcc-flags").split_whitespace() {
        args.push(OsStr::new(flag));
    }

    let status = Command::new("g++")
        .args(&args)
        .stdout(stdout())
        .stdin(Stdio::null())
        .stderr(stderr())
        .status()
        .wrap_err_with(|| "Error occured when call GCC !")?;

    if !status.success() {
        eyre::bail!(
            "GCC return : {:?} Source : {} Executable : {}",
            status,
            source.display(),
            executable.display()
        );
    }

    Ok(())
}
