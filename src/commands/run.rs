use crate::{clipboard, compile, run};
use clap::{ArgAction, Parser};
use eyre::eyre;
use std::path::PathBuf;
use std::{fs, path};

#[derive(Parser)]
pub struct RunArgs {
    #[arg(value_name = "SOURCE",default_value_t=String::from("."))]
    source: String,
    #[arg(value_name = "EXECUTABLE")]
    executable: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(short,long,action=ArgAction::SetTrue,value_name="CLIPBOARD")]
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
    let code_path = if source.is_dir() {
        let abs = path::absolute(&source).unwrap();
        let name = abs.file_name().unwrap().to_str().unwrap();
        let a = source.join(format!("{}.cpp", name));
        let b = source.join("p.cpp");
        let c = source.with_extension("cpp");
        if a.exists() {
            a
        } else if b.exists() {
            b
        } else if c.exists() {
            c
        } else {
            return Err(eyre!("No cpp file Found at {}", source.display()));
        }
    } else {
        if source.extension().is_none() {
            source.with_extension("cpp")
        } else {
            source
        }
    };
    let parent = code_path.parent().unwrap();
    let executable = match executable {
        Some(e) => PathBuf::from(e),
        None => {
            let executable = parent.join(code_path.file_stem().unwrap());
            #[cfg(target_os = "linux")]
            {
                executable
            }
            #[cfg(target_os = "windows")]
            {
                executable.with_extension("exe");
            }
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

    compile::compile(&code_path, &executable)?;

    let input = match (clipboard, file) {
        (true, None) => clipboard::get_clipboard()?,
        (false, Some(path)) => fs::read_to_string(path)?,
        (false, None) => {
            let store_path = parent.join("store.txt");
            let content = if store_path.exists() {
                fs::read_to_string(store_path)?
            } else {
                String::new()
            };
            content
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
