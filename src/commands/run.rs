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
    #[arg(short,long,action=ArgAction::SetTrue)]
    norun: bool,
    #[arg(short, long)]
    from_file: Option<PathBuf>,
    #[arg(short, long,action=ArgAction::SetTrue)]
    last_input: bool,
    #[arg(short,long,action=ArgAction::SetTrue)]
    store: bool,
}

pub fn run(args: RunArgs) -> eyre::Result<()> {
    let RunArgs {
        source,
        executable,
        norun,
        from_file,
        last_input,
        store,
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
    let store_path = source_parent.join("store.txt");
    let from_store = store_path.exists() && !store;
    let input = match (from_store, last_input) {
        (true, false) => fs::read_to_string(&store_path)?,
        (_, true) => fs::read_to_string(last_input_path)?,
        (false, false) => {
            let input = if let Some(file_path) = from_file {
                fs::read_to_string(file_path)?
            } else {
                clipboard::get_clipboard()?
            };
            fs::write(last_input_path, &input)?;
            input
        }
    };

    if store {
        fs::write(&store_path, &input)?;
    }

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
