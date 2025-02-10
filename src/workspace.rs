use std::{
    ffi::OsStr,
    fs,
    path::{self, Path, PathBuf},
};

#[derive(Debug, Clone, Default)]
pub struct Workspace {
    pub path: PathBuf,
    pub main_code: Option<PathBuf>,
    pub std_code: Option<PathBuf>,
    pub generator_code: Option<PathBuf>,
    pub cpp: Vec<PathBuf>,
    pub data: Vec<PathBuf>,
}

impl Workspace {
    pub fn new<P>(directory: P) -> eyre::Result<Self>
    where
        P: AsRef<Path>,
    {
        let directory = directory.as_ref();

        let find = |possible: &[&str]| -> Option<PathBuf> {
            for &name in possible {
                let path = directory.join(name);
                if path.exists() {
                    return Some(path);
                }
            }
            None
        };

        let find_main_code = || {
            let find_a = || {
                let abs = path::absolute(directory).ok()?;
                let name = abs.file_name()?.to_str()?;
                Some(directory.join(format!("{}.cpp", name)))
            };
            let a = find_a();
            let b = directory.join("p.cpp");
            if let Some(a) = a {
                if a.exists() {
                    return Some(a);
                }
            }
            Some(b)
        };

        let filter = |extension: &str| -> eyre::Result<Vec<PathBuf>> {
            let mut ret = Vec::new();
            for elm in fs::read_dir(directory)? {
                let entry = elm?;
                let path = entry.path();
                if path.extension() == Some(OsStr::new(extension)) {
                    ret.push(path);
                }
            }
            Ok(ret)
        };

        let main_code = find_main_code();
        let generator_code = find(&["gen.cpp"]);
        let std_code = find(&["bf.cpp", "std.cpp"]);

        let cpp = filter("cpp")?;
        let data = filter("in")?
            .into_iter()
            .map(|p| p.with_extension(""))
            .collect();

        Ok(Self {
            path: directory.to_path_buf(),
            main_code,
            generator_code,
            std_code,
            cpp,
            data,
        })
    }
}
