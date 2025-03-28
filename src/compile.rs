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

    let status = Command::new("g++")
        .args([
            source.as_os_str(),
            OsStr::new("-O2"),
            OsStr::new("-DDEBUG"),
            OsStr::new("-o"),
            executable.as_os_str(),
            OsStr::new("-Wall"),
            OsStr::new("-Wextra"),
            OsStr::new("-std=c++14"),
        ])
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
