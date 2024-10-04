use clap::Parser;

#[derive(Debug, Parser)]
pub struct NewArgs {
    #[clap(value_name = "NAME")]
    name: String,
    #[clap(short, long, value_name = "DIR", default_value_t=String::from("."))]
    directory: String,
    #[clap(short, long, default_value_t = false)]
    contest: bool,
    #[clap(short, long, default_value_t = 4)]
    problems_num: u32,
}

use std::fs;
use std::path::{Path, PathBuf};

const TEMPLATE: &str = include_str!("../../template/general.cpp");
const GEN_TEMPLATE: &str = include_str!("../../template/generator.cpp");
fn new_problem(name: &str, parent_directory: &Path) -> eyre::Result<()> {
    let directory = PathBuf::from(parent_directory).join(name);
    fs::create_dir_all(&directory)?;

    let program = directory.join("p.cpp");
    fs::write(program, TEMPLATE)?;

    let std_program = directory.join("std.cpp");
    fs::write(std_program, TEMPLATE)?;

    let generator = directory.join("gen.cpp");
    fs::write(generator, GEN_TEMPLATE)?;

    Ok(())
}

fn new_contest(name: &str, parent_directory: &Path, problems_num: u32) -> eyre::Result<()> {
    let directory = PathBuf::from(parent_directory).join(name);
    fs::create_dir_all(&directory)?;

    const CHAR_MAP: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    assert!(problems_num <= 26);

    let mut buffer: [u8; 128] = [0; 128];
    for problem in 0..problems_num {
        let name = CHAR_MAP[problem as usize].encode_utf8(&mut buffer);
        new_problem(name, &directory)?;
    }
    Ok(())
}

pub fn new(args: NewArgs) -> eyre::Result<()> {
    let NewArgs {
        name,
        directory,
        contest,
        problems_num,
    } = args;
    let directory = PathBuf::from(directory);
    if contest {
        new_contest(&name, &directory, problems_num)?;
    } else {
        new_problem(&name, &directory)?;
    }
    Ok(())
}
