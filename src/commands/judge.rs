use clap::Parser;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::{fs, path::PathBuf};

#[derive(Parser, Default)]
pub struct JudgeArgs {
    #[clap(short, long)]
    only: Option<PathBuf>,
    #[clap(default_value_t=String::from("./"))]
    directory: String,
}

pub fn judge(args: JudgeArgs) -> eyre::Result<()> {
    let JudgeArgs { only, directory } = args;
    
    if let Some(only)=only{
        assert!(directory.as_str()!="./","Directory and only should not be both set !");
    }

    let mut in_file = HashMap::new();
    let mut out_file = HashMap::new();

    for elm in read_dir(&directory)? {
        let entry = elm?;
        let path = entry.path();
        if path.extension() == Some(OsStr::new("in")) {
            in_file.insert(path.file_stem().unwrap().to_os_string(), path.clone());
        }
        if path.extension() == Some(OsStr::new("out")) {
            out_file.insert(path.file_stem().unwrap().to_os_string(), path.clone());
        }
    }
    
    for (name,path) in in_file{
        
    }
    Ok(())
}
