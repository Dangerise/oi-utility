use eyre::{ensure, Context, ContextCompat};

use super::*;

use std::ffi::OsStr;
use std::io::{stderr, stdout};
use std::path::{Path, PathBuf};
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
        ])
        .stdout(stdout())
        .stdin(Stdio::null())
        .stderr(stderr())
        .status()
        .wrap_err_with(|| "Error occured when call GCC !")?;

    if !status.success() {
        eyre::bail!("GCC return : {:?}", status);
    }

    Ok(())
}

pub fn compile_auto_name(path: impl AsRef<Path>) -> eyre::Result<PathBuf> {
    let path = path.as_ref();
    let directory = path
        .parent()
        .wrap_err_with(|| format!("Directory {} has no parent !", path.display()))?;

    let executable = directory.join(
        path.file_stem()
            .wrap_err_with(|| format!("Directory {} has no file name !", directory.display()))?,
    );

    compile::compile(path, &executable)?;

    Ok(executable)
}
