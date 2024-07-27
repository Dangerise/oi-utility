use crate::{clipboard, compile, run};
use clap::{ArgAction, Parser};
use eyre::eyre;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
pub struct RunArgs {
    #[arg(default_value_t = String::from("./p.cpp"), value_name = "SOURCE")]
    source: String,
    #[arg(value_name = "EXECUTABLE")]
    executable: Option<String>,
    #[arg(short, long)]
    file: Option<PathBuf>,
    #[arg(short,long,action=ArgAction::SetTrue)]
    clipboard: bool,
}

pub fn run(args: RunArgs) -> eyre::Result<()> {
    let RunArgs {
        source,
        executable,
        file,
        clipboard,
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

    let input = match (clipboard, file) {
        (true, None) => clipboard::get_clipboard()?,
        (false, Some(path)) => fs::read_to_string(path)?,
        (false, None) => {
            let store_path = source_parent.join("store.txt");
            fs::read_to_string(store_path)?
        }
        _ => {
            unreachable!()
        }
    };

    if input.len() < 500 {
        println!("Input:\n{}", &input);
    } else {
        println!("The input is too large");
    }

    println!("\nStdout:");

    let status = run::run(executable, input)?;

    println!("\n\nStatus:\n{:?}\n", status);
    Ok(())
}
