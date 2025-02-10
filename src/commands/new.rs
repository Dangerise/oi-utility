use clap::Parser;

#[derive(Debug, Parser)]
pub struct NewArgs {
    #[clap(value_name = "NAME")]
    name: String,
    #[clap(short, long, value_name = "DIR", default_value_t=String::from("."))]
    directory: String,
}

use std::fs;
use std::path::{Path, PathBuf};

const TEMPLATE: &str = include_str!("../../template/general.cpp");
const GEN_TEMPLATE: &str = include_str!("../../template/generator.cpp");
fn new_problem(name: &str, parent_directory: &Path) -> eyre::Result<()> {
    let directory = PathBuf::from(parent_directory).join(name);
    fs::create_dir_all(&directory)?;

    let program = directory.join(format!("{}.cpp", name));
    fs::write(program, TEMPLATE)?;

    let std_program = directory.join("std.cpp");
    fs::write(std_program, TEMPLATE)?;

    let generator = directory.join("gen.cpp");
    fs::write(generator, GEN_TEMPLATE)?;

    Ok(())
}

pub fn new(args: NewArgs) -> eyre::Result<()> {
    let NewArgs { name, directory } = args;
    let directory = PathBuf::from(directory);
    new_problem(&name, &directory)?;
    Ok(())
}
