use clap::Parser;

#[derive(Debug, Parser)]
pub struct NewArgs {
    #[clap(value_name = "NAME")]
    name: String,
    #[clap(short, long, value_name = "DIR", default_value_t=String::from("."))]
    path: String,
}

use std::fs;
use std::path::PathBuf;

const TEMPLATE: &str = include_str!("../../template/general.cpp");
const GEN_TEMPLATE: &str = include_str!("../../template/generator.cpp");
pub fn new(args: NewArgs) -> eyre::Result<()> {
    let path = PathBuf::from(args.path).join(args.name);
    fs::create_dir_all(&path)?;

    let program = path.join("p.cpp");
    fs::write(program, TEMPLATE)?;

    let std_program = path.join("std.cpp");
    fs::write(std_program, TEMPLATE)?;

    let generator = path.join("gen.cpp");
    fs::write(generator, GEN_TEMPLATE)?;

    Ok(())
}
