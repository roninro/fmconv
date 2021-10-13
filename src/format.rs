#![allow(dead_code)]

use std::str::FromStr;
use serde_yaml::Value as YamlValue;
use toml::Value as TomlValue;

use crate::errors;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
    Toml,
    Yaml,
}

impl Format {
    pub fn from_delim(d: &str) -> Result<Self, errors::Error> {
        match d {
            "+++" => Ok(Format::Toml),
            "---" => Ok(Format::Yaml),
            _ => Err(errors::Error::FormatName(d.to_string())),
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Format::Toml => "toml",
            Format::Yaml => "yaml",
        }
    }

    pub fn delimiter(&self) -> &'static str {
        match *self {
            Format::Toml => "+++",
            Format::Yaml => "---",
        }
    }

    pub fn front_matter(&self, s: &str) -> Box<dyn ToFormat> {
        match self {
            Format::Toml => Box::new(TomlMatter(s.to_string())),
            Format::Yaml => Box::new(YamlMatter(s.to_string())),
        }
    }
}

impl FromStr for Format {
    type Err = errors::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lower = s.to_ascii_lowercase();
        match lower.as_str() {
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(errors::Error::FormatName(lower)),
        }
    }
}

pub fn convert_to(s: &str, src: Format, dst: Format) -> Result<String, errors::Error> {
    let fr = src.front_matter(s);
    fr.to(dst)
}

pub trait ToFormat {
    fn to_toml(&self) -> Result<String, errors::Error>;
    fn to_yaml(&self) -> Result<String, errors::Error>;
    fn to(&self, f: Format) -> Result<String, errors::Error> {
        match f {
            Format::Toml => self.to_toml(),
            Format::Yaml => self.to_yaml(),
        }
    }
}

pub struct TomlMatter(String);

impl ToFormat for TomlMatter {
    fn to_toml(&self) -> Result<String, errors::Error> {
        Ok(self.0.clone())
    }
    fn to_yaml(&self) -> Result<String, errors::Error> {
        let v: TomlValue =
            toml::from_str(&self.0).map_err(|e| errors::Error::Deserialization(e.to_string()))?;
        let yaml = convert_toml_to_yaml(v);

        if let YamlValue::Mapping(m) = &yaml {
            if m.is_empty() {
                return Ok("---\n\n".to_string());
            }
        }

        let r = serde_yaml::to_string(&yaml)
            .map_err(|e| errors::Error::Serialization(e.to_string()))?;
        Ok(r)
    }
}

pub struct YamlMatter(String);

impl ToFormat for YamlMatter {
    fn to_yaml(&self) -> Result<String, errors::Error> {
        Ok(self.0.clone())
    }
    fn to_toml(&self) -> Result<String, errors::Error> {
        let v: YamlValue = serde_yaml::from_str(&self.0)
            .map_err(|e| errors::Error::Deserialization(e.to_string()))?;
        let r =
            toml::to_string_pretty(&v).map_err(|e| errors::Error::Serialization(e.to_string()))?;
        Ok(r)
    }
}

fn convert_toml_to_yaml(toml: TomlValue) -> YamlValue {
    match toml {
        TomlValue::String(s) => YamlValue::String(s),
        TomlValue::Integer(i) => YamlValue::Number(i.into()),
        TomlValue::Float(f) => {
            let n = serde_yaml::Number::from(f);
            YamlValue::Number(n)
        }
        TomlValue::Boolean(b) => YamlValue::Bool(b),
        TomlValue::Array(arr) => {
            YamlValue::Sequence(arr.into_iter().map(convert_toml_to_yaml).collect())
        }
        TomlValue::Table(table) => {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in table
                .into_iter()
                .map(|(k, v)| (k, convert_toml_to_yaml(v)))
            {
                map.insert(YamlValue::String(k), v);
            }
            YamlValue::Mapping(map)
        }
        TomlValue::Datetime(dt) => YamlValue::String(dt.to_string()),
    }
}
