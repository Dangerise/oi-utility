use crate::workspace::Workspace;
use crate::{clipboard, compile, run};
use clap::{ArgAction, Parser};
use eyre::{ensure, Context, ContextCompat};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
pub struct RunArgs {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short,long,action=ArgAction::SetTrue,value_name="CLIPBOARD")]
    clipboard: bool,
}

pub fn run(workspace: Workspace, args: RunArgs) -> eyre::Result<()> {
    let RunArgs {
        file,
        clipboard,
        output,
    } = args;
    let Workspace {
        path, main_code, ..
    } = workspace;

    let main_code = main_code.wrap_err("solution code is not given !")?;

    ensure!(
        main_code.exists(),
        "path \"{}\" do not exist !",
        main_code.display()
    );

    let executable = {
        let executable =
            path.join(main_code.file_stem().wrap_err_with(|| {
                format!("path \"{}\" has no file stem !", main_code.display())
            })?);
        #[cfg(target_os = "linux")]
        {
            executable
        }
        #[cfg(target_os = "windows")]
        {
            executable.with_extension("exe");
        }
    };

    println!(
        "Compiling {} to {} ...\n",
        main_code.display(),
        executable.display()
    );

    compile::compile(&main_code, &executable)?;

    let input = match (clipboard, file) {
        (true, None) => clipboard::get_clipboard()?,
        (false, Some(path)) => fs::read_to_string(path)?,
        (false, None) => {
            let store_path = path.join("store.in");
            
            if store_path.exists() {
                fs::read_to_string(store_path)?
            } else {
                String::new()
            }
        }
        _ => {
            unreachable!()
        }
    };

    if input.len() < 500 {
        println!("Input:\n{}", &input);
    } else {
        println!("Input size = {}", input.len());
    }

    let status = if let Some(output) = output {
        let result = run::run_silently(executable, &input)?;
        fs::write(&output, result.output)
            .wrap_err_with(|| format!("Error occured when writing to \"{}\"", output.display()))?;
        result.status
    } else {
        println!("\nStdout:");
        run::run(executable, input)?
    };

    println!("\n\nStatus:\n{:?}\n", status);
    Ok(())
}
