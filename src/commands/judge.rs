use clap::Parser;
use eyre::{Context, ContextCompat, Ok};
use std::fs::{self};
use std::path::PathBuf;

use crate::workspace::Workspace;
use crate::{compile, executable_path, judge, run};

#[derive(Parser, Default)]
pub struct JudgeArgs {
    #[clap(short, long)]
    only: Option<PathBuf>,
}

pub fn judge(workspace: Workspace, args: JudgeArgs) -> eyre::Result<()> {
    let Workspace {
        main_code,
        mut data,
        ..
    } = workspace;

    let main_code = main_code.wrap_err("main code is not given !")?;
    let JudgeArgs { only } = args;
    if let Some(path) = only {
        data = vec![path];
    }

    let executable = executable_path(&main_code)?;
    compile::compile(&main_code, &executable)?;

    for in_file in data {
        let input = fs::read_to_string(&in_file)
            .wrap_err_with(|| format!("Error occured while reading \"{}\"", in_file.display()))?;
        println!("Running {}\n", in_file.display());
        let result = run::run_silently(&executable, &input)?;
        let out_file = in_file.with_extension("out");
        if out_file.exists() {
            let ans = fs::read_to_string(&out_file).wrap_err_with(|| {
                format!("Error occured while reading \"{}\"", out_file.display())
            })?;
            if judge::judge(&result.output, &ans) {
                println!("Passed\n");
            } else {
                println!("Failed !");
                return Ok(());
            }
        }
    }
    Ok(())
}
