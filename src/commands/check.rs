use crate::{compile, judge, run};

use clap::Parser;
use eyre::eyre;
use std::fs;

#[derive(Parser)]
pub struct CheckArgs {
    #[clap(short,long,default_value_t=String::from("./p.cpp"))]
    my: String,
    #[clap(short,long,default_value_t=String::from("./std.cpp"))]
    std: String,
    #[clap(short,long,default_value_t=String::from("./gen.cpp"))]
    generator_code: String,
    #[clap(short,long,default_value_t=String::from("./input.txt"))]
    input: String,
    #[clap(short, long, default_value_t = 100)]
    times: u32,
}

// fn single_task(index:u32)->bo

pub fn check(args: CheckArgs) -> eyre::Result<()> {
    println!(
        "Compiling gen : {} std: {} my : {}...",
        &args.generator_code, &args.std, &args.my
    );

    let my_path = compile::compile_auto_name(&args.my)?;
    let std_path = compile::compile_auto_name(&args.std)?;
    let generator_path = compile::compile_auto_name(&args.generator_code)?;
    for i in 0..args.times {
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

        let input = fs::read_to_string("input.txt")?;

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
