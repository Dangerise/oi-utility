use std::fs;

use clap::Parser;
use eyre::eyre;

use crate::{
    compile::compile_auto_name,
    run::{self, run_silently},
};

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

fn judge(a: &str, b: &str) -> bool {
    let mut a_lines = a.lines();
    let mut b_lines = b.lines();

    // fn iter<A, B, F>(a: A, b: B, pred: F) -> bool
    // where
    //     A: for<'a> Iterator<Item = &'a str>,
    //     B: for<'b> Iterator<Item = &'b str>,
    //     F: FnMut(&str, &str) -> bool,
    // {
    //     todo!()
    // }

    loop {
        let a = a_lines.next();
        let b = b_lines.next();
        if a.is_none() ^ b.is_none() {
            return false;
        }
        if a.is_none() && b.is_none() {
            return true;
        }
        let mut a = a.unwrap().split_whitespace();
        let mut b = b.unwrap().split_whitespace();
        loop {
            let a = a.next();
            let b = b.next();
            if a.is_none() ^ b.is_none() {
                return false;
            }
            if a.is_none() && b.is_none() {
                break;
            }
            let a = a.unwrap();
            let b = b.unwrap();
            if a != b {
                return false;
            }
        }
    }
}

// fn single_task(index:u32)->bo

pub fn check(args: CheckArgs) -> eyre::Result<()> {
    println!(
        "Compiling gen : {} std: {} my : {}...",
        &args.generator_code, &args.std, &args.my
    );

    let my_path = compile_auto_name(&args.my)?;
    let std_path = compile_auto_name(&args.std)?;
    let generator_path = compile_auto_name(&args.generator_code)?;
    for i in 0..args.times {
        println!("Task #{}", i);

        println!("Running data generator at {} ...", generator_path.display());

        let input_result = run_silently(&generator_path, "input")?;
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

        if judge(&my_output.output, &std_output.output) {
            println!("Passed !\n");
        } else {
            println!("Failed !\n");
            return Ok(());
        }
    }

    Ok(())
}
