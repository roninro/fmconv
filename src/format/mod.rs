use std::str::FromStr;

use serde::de;  
use crate::errors;

mod toml;
mod yaml;

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

pub struct FrontMatter {
    format: Format,
    pub text: String,
}

impl FrontMatter {
    pub fn new(format: Format, text: String) -> FrontMatter {
        FrontMatter { format, text }
    }

    pub fn convert_to(&self, format: Format) -> Result<FrontMatter, errors::Error> {
        match format {
            Format::Toml => self.to_toml(),
            Format::Yaml => self.to_yaml(),
        }
        .map(|text| FrontMatter { text, format })
    }

    fn to_toml(&self) -> Result<String, errors::Error> {
        // let v = self.deserialize::<toml::InnerValue>(&self.text)?;
        let v = toml::deserialize(&self.text)?;
        toml::serialize(&v)
    }

    fn to_yaml(&self) -> Result<String, errors::Error> {
        // let v = self.deserialize::<yaml::InnerValue>(&self.text)?;
        let v = yaml::deserialize(&self.text)?;
        yaml::serialize(&v)
    }
    fn deserialize<'de, T>(&self, s: &str) -> Result<T, errors::Error>
    where
        T: de::Deserialize<'de>,
    {
        match self.format {
            Format::Toml => toml::deserialize(s),
            Format::Yaml => yaml::deserialize(s),
        }
    }
}
