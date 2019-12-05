use failure::Error;
use std::env::current_dir;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
enum ExecMode {
    Stdin,
    File,
}

#[derive(Serialize, Deserialize)]
pub struct Conf {
    year: i32,
    editor: String,
    exec: Option<String>,
    mode: Option<ExecMode>,
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
}

/// Find configuration directory in current directory or its ancestors
pub fn find() -> Result<PathBuf, Error> {
    let cwd = current_dir()?;

    let conf_dir = cwd.ancestors()
        .find(|dir| dir.join(".aocf").is_dir())
        .filter(|dir| dir.join(".aocf/config").is_file());

    match conf_dir {
        Some(dir) => Ok(dir.to_path_buf()),
        None => bail!("no aocf configuration found"),
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
    fn test_find() {
        let tmp = tempdir().unwrap();
        let tmp_path = tmp.path();
        let tmp_sub = tmp_path.join("im/in/a-subdir");
        fs::create_dir_all(&tmp_sub).unwrap();

        env::set_current_dir(tmp_path);
        assert!(find().is_err());
        fs::create_dir(tmp_path.join(".aocf"));
        assert!(find().is_err());
        let mut conf = File::create(tmp_path.join(".aocf/config"));
        assert!(find().is_ok());
        env::set_current_dir(tmp_sub);
        assert_eq!(find().unwrap(), tmp_path);
    }
}
