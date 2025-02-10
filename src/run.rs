use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

#[derive(Debug, Clone)]
pub struct RunResult {
    pub status: ExitStatus,
    pub output: String,
}

use eyre::Context;


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
        .spawn()
        .wrap_err_with(|| format!("Errro occured when call \"{}\" !", executable.display()))?;

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
        eyre::bail!("Run {} status {:?}", executable.display(), status)
    }

    Ok(status)
}
