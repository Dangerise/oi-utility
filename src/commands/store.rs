use std::{fs, path::PathBuf};

use super::*;

use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct StoreArgs {
    #[clap(short, long, default_value_t = true)]
    clipboard: bool,
    #[clap(short, long)]
    file: Option<PathBuf>,
}

pub fn store(args: StoreArgs) -> eyre::Result<()> {
    let StoreArgs { clipboard, file } = args;

    let content = match (clipboard, file) {
        (true, None) => clipboard::get_clipboard()?,
        (false, Some(path)) => fs::read_to_string(path)?,
        _ => {
            panic!()
        }
    };

    fs::write("./store.txt", content)?;

    Ok(())
}
