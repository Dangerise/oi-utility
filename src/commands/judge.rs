use clap::Parser;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Parser, Default)]
pub struct JudgeArgs {
    #[clap(short, long)]
    only: Option<PathBuf>,
    #[clap(default_value_t=String::from("./"))]
    directory: String,
    #[clap(short, long, default)]
    source: String,
    #[arg(value_name = "EXECUTABLE")]
    executable: Option<String>,
}

pub fn judge(args: JudgeArgs) -> eyre::Result<()> {
    let JudgeArgs { only, directory } = args;
    let directory = PathBuf::from(directory);

    if only.is_some() {
        assert!(
            directory.as_os_str() != OsStr::new("./"),
            "Directory and only should not be both set !"
        );
    }

    let mut tasks = Vec::new();

    for elm in read_dir(&directory)? {
        let entry = elm?;
        let path = entry.path();
        if path.extension() == Some(OsStr::new("in")) {
            let file_stem = path.file_stem().unwrap();
            tasks.push(directory.join(file_stem));
        }
    }

    for task in tasks {
        let in_file = task;
    }

    Ok(())
}
