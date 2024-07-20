use super::*;
use std::env::consts::FAMILY;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

use clap::{ArgAction, Parser};
#[derive(Parser)]
pub struct RunArgs {
    #[arg(default_value_t = String::from("./p.cpp"), value_name = "SOURCE")]
    source: String,
    #[arg(value_name = "EXECUTABLE")]
    executable: Option<String>,
    #[arg(short,long,action=ArgAction::SetTrue)]
    norun: bool,
    #[arg(short, long)]
    from_file: Option<PathBuf>,
    #[arg(short, long,action=ArgAction::SetTrue)]
    last_input: bool,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub status: ExitStatus,
    pub output: String,
}

use compile::compile_auto_name;
pub fn compile_and_run_silently(path: impl AsRef<Path>, input: &str) -> eyre::Result<RunResult> {
    let executable = compile_auto_name(path)?;

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

fn run_inside(executable: impl AsRef<Path>, input: String) -> eyre::Result<ExitStatus> {
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

pub fn run(args: RunArgs) -> eyre::Result<()> {
    let RunArgs {
        source,
        executable,
        norun,
        from_file,
        last_input,
    } = args;

    let source = PathBuf::from(&source);

    let code_path = if source.extension().is_none() {
        source.with_extension("cpp")
    } else {
        source.clone()
    };

    let source_parent = source.parent().unwrap();
    let executable = match executable {
        Some(e) => PathBuf::from(e),
        None => {
            let executable;
            #[cfg(target_os = "linux")]
            {
                executable = source_parent.join("p");
            }
            #[cfg(target_os = "windows")]
            {
                executable = source_parent.push("p.exe");
            }
            executable
        }
    };

    if !code_path.exists() {
        return Err(eyre!("Code not {} no exist !", code_path.display()));
    }

    println!(
        "Compiling {} to {} ...\n",
        code_path.display(),
        executable.display()
    );

    compile::compile(code_path, &executable)?;

    if norun {
        return Ok(());
    }

    let last_input_path = source_parent.join("last_input.txt");
    let input = match last_input {
        true => fs::read_to_string(last_input_path)?,
        false => {
            let input = if let Some(file_path) = from_file {
                fs::read_to_string(file_path)?
            } else {
                clipboard::get_clipboard()?
            };
            fs::write(last_input_path, &input)?;
            input
        }
    };

    if input.len() < 500 {
        println!("Input:\n{}", &input);
    } else {
        println!("The input is too large");
    }

    println!("\nStdout:");

    let status = run_inside(executable, input)?;

    println!("\n\nStatus:\n{:?}\n", status);
    Ok(())
}
