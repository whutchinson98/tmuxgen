use anyhow::Context;

use crate::constants::CONFIG_FILE_NAME;

#[derive(Debug, serde::Deserialize)]
pub struct Config {}

impl Config {
    /// Load the config from the specified path
    #[tracing::instrument(level = "trace")]
    fn load_from_file(path: &str) -> anyhow::Result<String> {
        let content = std::fs::read(path)?;
        tracing::trace!("read config file");
        let content = String::from_utf8(content)?;
        Ok(content)
    }

    /// Load the config from the current working directory
    #[tracing::instrument(level = "trace")]
    fn load_from_cwd() -> anyhow::Result<String> {
        let file_path = std::env::current_dir()?.join(CONFIG_FILE_NAME);

        let file_path = file_path
            .to_str()
            .context("failed to convert path to string")?;

        tracing::trace!(
            file_path,
            "created file path from current working directory"
        );

        let content = Self::load_from_file(file_path)?;

        Ok(content)
    }

    /// Load the config from personal working directory or from a specified path
    #[tracing::instrument(level = "trace")]
    pub fn from_file(override_path: Option<&str>) -> anyhow::Result<Self> {
        let file_content = match override_path {
            Some(path) => Self::load_from_file(path),
            None => Self::load_from_cwd(),
        }?;

        tracing::trace!(file_content, "loaded config file content");

        let config: Config = toml::from_str(&file_content)?;

        Ok(config)
    }
}
