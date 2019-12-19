use failure::Error;
use std::collections::hash_map::DefaultHasher;
use std::env::current_dir;
use std::fs::{File, read_to_string};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Hash)]
enum ExecMode {
    Stdin,
    File,
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Conf {
    pub year: i32,
    pub day: u32,
    editor: String,
    pub pager: String,
    exec: Option<String>,
    mode: Option<ExecMode>,
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            year: 2015,
            day: 1,
            editor: "vim".into(),
            pager: "less".into(),
            exec: None,
            mode: None,
        }
    }
}

impl Conf {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let conf = read_to_string(path.as_ref())?;
        let conf: Self = toml::de::from_str(&conf)?;
        Ok(conf)
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut file = File::create(path)?;
        file.write_all(toml::ser::to_string(&self)?.as_bytes())?;
        Ok(())
    }

    pub fn calc_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

/// Find configuration directory in current directory or its ancestors
pub fn find_root() -> Result<PathBuf, Error> {
    let cwd = current_dir()?;

    let conf_dir = cwd.ancestors()
        .find(|dir| dir.join(".aocf").is_dir())
        .filter(|dir| dir.join(".aocf/config").is_file());

    match conf_dir {
        Some(dir) => Ok(dir.to_path_buf()),
        None => bail!("no configuration found"),
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;
    use super::*;
    use self::tempfile::tempdir;
    use std::env;
    use std::fs;

    #[test]
    fn test_find_root() {
        let tmp = tempdir().unwrap();
        let tmp_path = tmp.path();
        let tmp_sub = tmp_path.join("im/in/a-subdir");
        fs::create_dir_all(&tmp_sub).unwrap();

        env::set_current_dir(tmp_path).unwrap();
        assert!(find_root().is_err());
        fs::create_dir(tmp_path.join(".aocf")).unwrap();
        assert!(find_root().is_err());
        File::create(tmp_path.join(".aocf/config")).unwrap();
        assert!(find_root().is_ok());
        env::set_current_dir(tmp_sub).unwrap();
        assert_eq!(find_root().unwrap(), tmp_path);
    }
}
