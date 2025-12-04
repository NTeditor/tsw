use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;

pub struct Config {
    pub name: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub version: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let metadata = MetadataCommand::new().exec()?;
        let root_package = metadata.root_package().context("Failed get root_package")?;
        Ok(Self {
            name: root_package.name.to_string(),
            authors: root_package.authors.clone(),
            description: root_package.description.clone(),
            version: root_package.version.to_string(),
        })
    }
}
