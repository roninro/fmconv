use serde::{Deserialize, Serialize};
use serde::de;
use crate::errors;

pub use toml::Value as InnerValue;

pub fn serialize<V: Serialize>(v: V) -> Result<String, errors::Error> {
    let toml = toml::to_string(&v).map_err(|e| errors::Error::Serialization(e.to_string()))?;
    Ok(toml)
}


pub fn deserialize(s: &str) -> Result<InnerValue, errors::Error> {
    let mut data: InnerValue =
        toml::from_str(s).map_err(|e| errors::Error::Deserialization(e.to_string()))?;
    let t_map = data.as_table_mut().unwrap();
    for (_k, v) in t_map.iter_mut() {
        // println!("{:?} {:?}", k, v);
        if let InnerValue::Datetime(t) = v {
            *v = InnerValue::String(t.to_string());
        }
    }
    Ok(data)

}
