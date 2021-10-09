use std::str::FromStr;

use crate::errors;

// mod toml;
// mod yaml;


#[derive(Debug, Copy, Clone)]
pub enum Format {
    Toml,
    Yaml,
}


impl Format {

    pub fn name(&self) -> &'static str {
        match *self {
            Format::Toml => "toml",
            Format::Yaml => "yaml",
        }
    }

    pub fn extensions(&self) -> &[&'static str] {
        match *self {
            Format::Toml => &["toml"],
            Format::Yaml => &["yaml", "yml"],
        }
    }

    pub fn is_extension(&self, s: &str) -> bool {
        self.extensions().iter().find(|&&ext| ext == s).is_some()
    }

    pub fn preferred_extension(&self) -> &'static str {
        self.name()
    }
}

impl FromStr for Format {
    type Err = errors::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lower = s.to_ascii_lowercase();
        match lower.as_str() {
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(errors::Error::FormatName(lower))
        }
    }
}

pub struct Frontmatter {
    format: Format,
    pub text: String,
}









