pub mod commands;

pub mod compile;

pub mod clipboard;

pub mod run;

pub mod judge;

pub mod workspace;

use std::path::{self, Path, PathBuf};

use eyre::{ContextCompat, Ok};
pub fn executable_path<P: AsRef<Path>>(path: P) -> eyre::Result<PathBuf> {
    let path = path.as_ref();
    let parent = path::absolute(path)?;
    let executable = parent.join(
        path.file_stem()
            .wrap_err_with(|| format!("Path \"{}\" has not file stem !", path.display()))?,
    );
    let executable = {
        #[cfg(target_os = "linux")]
        {
            executable
        }
        #[cfg(target_os = "windows")]
        {
            executable.with_extension("exe");
        }
    };
    Ok(executable)
}
