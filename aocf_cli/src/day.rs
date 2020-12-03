use failure::Error;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};
use aocf::Level;

#[derive(Serialize, Deserialize, Default)]
pub struct Day {
    #[serde(serialize_with = "ordered_map")]
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

/// Get an ordered hashmap representation when serialising
pub fn ordered_map<S>(value: &HashMap<Level, Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
