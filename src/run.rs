use std::io::{self, Write};
use std::path::{Path};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;


#[derive(Debug, Clone)]
pub struct RunResult {
    pub status: ExitStatus,
    pub output: String,
}

use super::{compile, run};
pub fn compile_and_run_silently(path: impl AsRef<Path>, input: &str) -> eyre::Result<RunResult> {
    let executable = compile::compile_auto_name(path)?;

    let result = run::run_silently(executable, input)?;

    Ok(result)
}

pub fn run_silently(executable: impl AsRef<Path>, input: &str) -> eyre::Result<RunResult> {
    let executable = executable.as_ref();
    let mut child = Command::new(executable)
        .stdin(if input.is_empty() {
            Stdio::null()
        } else {
            Stdio::piped()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    if !input.is_empty() {
        let input = input.to_string();
        let handle = thread::spawn(move || -> eyre::Result<()> {
            stdin.write_all(input.as_bytes())?;
            Ok(())
        });
        handle.join().unwrap()?;
    }

    let result = child.wait_with_output()?;
    let output = result.stdout;
    let output = String::from_utf8(output)?;
    let status = result.status;

    Ok(RunResult { status, output })
}

pub fn run(executable: impl AsRef<Path>, input: String) -> eyre::Result<ExitStatus> {
    let executable = executable.as_ref();
    // dbg!(executable);
    let mut child = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(io::stdout())
        .stderr(io::stderr())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let handle = thread::spawn(move || -> eyre::Result<()> {
        stdin.write_all(input.as_bytes())?;
        Ok(())
    });
    handle.join().unwrap()?;

    let status = child.wait()?;

    if !status.success() {
        eyre::bail!("{:?}", status)
    }

    Ok(status)
}
