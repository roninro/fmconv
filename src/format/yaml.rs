use serde::{Deserialize, Serialize};

use crate::errors;

pub use serde_yaml::Value as InnerValue;

pub fn serialize<V: Serialize>(v: V) -> Result<String, errors::Error> {
    let toml =
        serde_yaml::to_string(&v).map_err(|e| errors::Error::Serialization(e.to_string()))?;
    Ok(toml)
}

pub fn deserialize<V>(s: &str) -> Result<V, errors::Error>
where
    V: for<'de> Deserialize<'de>,
{
    let data =
        serde_yaml::from_str(s).map_err(|e| errors::Error::Deserialization(e.to_string()))?;
    Ok(data)
}
