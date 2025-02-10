use crate::{compile, executable_path, judge, run, workspace::Workspace};

use clap::Parser;
use eyre::{eyre, ContextCompat};
use std::fs;

#[derive(Parser)]
pub struct CheckArgs {
    #[clap(short,long,default_value_t=String::from("./.in"))]
    input: String,
    #[clap(short, long, default_value_t = 100)]
    times: u32,
}

// fn single_task(index:u32)->bo

pub fn check(workspace: Workspace, args: CheckArgs) -> eyre::Result<()> {
    let Workspace {
        main_code,
        std_code,
        generator_code,
        ..
    } = workspace;
    let CheckArgs { input, times } = args;

    let main_code = main_code.wrap_err("main code is not given !")?;
    let std_code = std_code.wrap_err("std code is not given !")?;
    let generator_code = generator_code.wrap_err("generator code is not given !")?;

    println!(
        "Compiling gen : {} std: {} my : {}...",
        &generator_code.display(),
        &std_code.display(),
        &main_code.display()
    );

    let my_path = executable_path(&main_code)?;
    let std_path = executable_path(&std_code)?;
    let generator_path = executable_path(&generator_code)?;

    compile::compile(&main_code, &my_path)?;
    compile::compile(&std_code, &std_path)?;
    compile::compile(&generator_code, &generator_path)?;

    for i in 0..times {
        println!("Task #{}", i);

        println!("Running data generator at {} ...", generator_path.display());

        let input_result = run::run_silently(&generator_path, "input")?;
        if !input_result.status.success() {
            return Err(eyre!(
                "Generator : {}\nOutput:{}\n",
                input_result.status,
                input_result.output
            ));
        }

        let input = fs::read_to_string(&input)?;

        println!("Running my program at {} ...", my_path.display());
        let my_output = run::run_silently(&my_path, &input)?;

        println!("Running std program at {} ...", std_path.display());
        let std_output = run::run_silently(&std_path, &input)?;

        if !my_output.status.success() {
            return Err(eyre!("{}", my_output.status));
        }
        if !std_output.status.success() {
            return Err(eyre!("{}", std_output.status));
        }

        if judge::judge(&my_output.output, &std_output.output) {
            println!("Passed !\n");
        } else {
            println!("Failed !\n");
            if input.len() < 100 && std_output.output.len() < 100 && my_output.output.len() < 100 {
                println!(
                    "Input:\n{}\n\nstd:\n{}\n\nmy:\n{}\n",
                    &input, std_output.output, my_output.output
                );
            }
            return Ok(());
        }
    }

    Ok(())
}
