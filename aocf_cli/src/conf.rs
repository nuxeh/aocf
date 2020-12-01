use failure::Error;
use std::collections::hash_map::DefaultHasher;
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
