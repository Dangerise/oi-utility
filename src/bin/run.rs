use oi_utility::*;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

struct CliArgs {
    source: PathBuf,
    executable: PathBuf,
}
fn parse_arg() -> eyre::Result<CliArgs> {
    use clap::*;
    let matches = Command::new("runner")
        .arg(Arg::new("source").required(true))
        .arg(
            Arg::new("executable")
                .short('e')
                .long("exe")
                .default_value("./bin/"),
        )
        .get_matches();
    let source = matches.get_one::<String>("source").unwrap().clone();
    let mut source = PathBuf::from(source);
    if source.extension() != Some(OsStr::new("cpp")) {
        source = source.with_extension("cpp");
    }

    let mut executable: PathBuf = matches
        .get_one::<String>("executable")
        .unwrap()
        .clone()
        .into();
    let file_name = source.file_stem().unwrap();
    if executable.exists() {
        if executable.is_dir() {
            executable.push(file_name);
        }
    } else {
        fs::create_dir_all(&executable)?;
        executable.push(file_name);
    }

    let args = CliArgs { source, executable };
    Ok(args)
}

fn run(executable: impl AsRef<Path>, input: String) -> eyre::Result<ExitStatus> {
    let executable = executable.as_ref();
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
fn main() -> eyre::Result<()> {
    let args = parse_arg()?;

    println!(
        "Compiling {} to {} ...\n",
        args.source.display(),
        args.executable.display()
    );

    compile::compile(&args.source, &args.executable)?;

    let clipboard = clipboard::get_clipboard()?;
    if clipboard.len() < 500 {
        println!("Clipboard:\n{}", &clipboard);
    } else {
        println!("The clipboard is too large");
    }

    println!("\nStdout:");

    let status = run(&args.executable, clipboard)?;

    println!("\nStatus:\n{:?}", status);
    Ok(())
}
