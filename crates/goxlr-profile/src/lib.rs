use directories::ProjectDirs;
use goxlr_model::{AppSettingsSummary, LogLevel};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct AppConfig {
    pub mock_device_enabled: bool,
    pub show_tray_icon: bool,
    pub start_minimized: bool,
    pub launch_at_startup: bool,
    pub log_level: LogLevel,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mock_device_enabled: false,
            show_tray_icon: true,
            start_minimized: false,
            launch_at_startup: false,
            log_level: LogLevel::Info,
        }
    }
}

impl AppConfig {
    pub fn summary(&self, config_path: Option<String>) -> AppSettingsSummary {
        AppSettingsSummary {
            mock_device_enabled: self.mock_device_enabled,
            show_tray_icon: self.show_tray_icon,
            start_minimized: self.start_minimized,
            launch_at_startup: self.launch_at_startup,
            log_level: self.log_level,
            config_path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_for_app() -> Result<Self, ConfigError> {
        Ok(Self::new(default_config_path()?))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load_or_default(&self) -> Result<AppConfig, ConfigError> {
        match self.load() {
            Ok(config) => Ok(config),
            Err(ConfigError::NotFound) => Ok(AppConfig::default()),
            Err(error @ ConfigError::Malformed { .. }) => {
                tracing::warn!(%error, "configuration is malformed; using defaults");
                Ok(AppConfig::default())
            }
            Err(error) => Err(error),
        }
    }

    pub fn load(&self) -> Result<AppConfig, ConfigError> {
        let contents = fs::read_to_string(&self.path).map_err(|source| {
            if source.kind() == io::ErrorKind::NotFound {
                ConfigError::NotFound
            } else {
                ConfigError::Io { source }
            }
        })?;

        serde_json::from_str(&contents).map_err(|source| ConfigError::Malformed {
            path: self.path.clone(),
            source,
        })
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), ConfigError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|source| ConfigError::Io { source })?;
        }

        let contents = serde_json::to_string_pretty(config)
            .map_err(|source| ConfigError::Serialize { source })?;
        fs::write(&self.path, contents).map_err(|source| ConfigError::Io { source })
    }
}

pub fn default_config_dir() -> Result<PathBuf, ConfigError> {
    let dirs = ProjectDirs::from("dev", "FriesXLR", "FriesXLR")
        .ok_or(ConfigError::ProjectDirectoryUnavailable)?;
    Ok(dirs.config_dir().to_path_buf())
}

pub fn default_config_path() -> Result<PathBuf, ConfigError> {
    Ok(default_config_dir()?.join("config.json"))
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("configuration file was not found")]
    NotFound,
    #[error("could not determine the application configuration directory")]
    ProjectDirectoryUnavailable,
    #[error("configuration file {path} is malformed: {source}")]
    Malformed {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("configuration serialization failed: {source}")]
    Serialize { source: serde_json::Error },
    #[error("configuration file I/O failed: {source}")]
    Io { source: io::Error },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_config_loads_default() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));

        assert_eq!(store.load_or_default().unwrap(), AppConfig::default());
    }

    #[test]
    fn malformed_config_is_not_fatal() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        fs::write(&path, "{not-json").unwrap();
        let store = ConfigStore::new(path);

        assert_eq!(store.load_or_default().unwrap(), AppConfig::default());
    }

    #[test]
    fn saves_and_loads_config() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));
        let config = AppConfig {
            mock_device_enabled: true,
            ..AppConfig::default()
        };

        store.save(&config).unwrap();

        assert_eq!(store.load().unwrap(), config);
    }
}
