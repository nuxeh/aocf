use failure::Error;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use aocf::Level;

#[derive(Serialize, Deserialize, Default)]
pub struct Day {
    pub exec: HashMap<Level, Vec<String>>,
}

impl Day {
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
