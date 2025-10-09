//! Configuration file discovery
//!
//! This module handles automatic discovery of `codegen.toml` configuration files
//! by searching in multiple locations following standard conventions.

use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};

/// Default configuration file name
pub const CONFIG_FILENAME: &str = "codegen.toml";

/// XDG configuration directory name
pub const XDG_CONFIG_DIR: &str = "octofhir";

/// Configuration file discovery result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryResult {
    /// Configuration file found at the specified path
    Found(PathBuf),
    /// No configuration file found
    NotFound,
}

impl DiscoveryResult {
    /// Check if a configuration file was found
    pub fn is_found(&self) -> bool {
        matches!(self, DiscoveryResult::Found(_))
    }

    /// Get the path if found
    pub fn path(&self) -> Option<&Path> {
        match self {
            DiscoveryResult::Found(path) => Some(path),
            DiscoveryResult::NotFound => None,
        }
    }

    /// Unwrap the path, panicking if not found
    pub fn unwrap(&self) -> &Path {
        self.path().expect("Called unwrap on NotFound")
    }

    /// Convert to Result
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<PathBuf, E> {
        match self {
            DiscoveryResult::Found(path) => Ok(path),
            DiscoveryResult::NotFound => Err(err()),
        }
    }
}

/// Discover configuration file using multiple search strategies
///
/// Search order:
/// 1. Explicit path (if provided)
/// 2. Current directory
/// 3. Parent directories (walking up the tree)
/// 4. XDG Base Directory (~/.config/octofhir/codegen.toml)
/// 5. Home directory (~/.octofhir/codegen.toml)
///
/// # Examples
///
/// ```no_run
/// use octofhir_codegen::cli::discovery::discover_config;
/// use std::path::PathBuf;
///
/// // Auto-discover configuration
/// let result = discover_config(None::<PathBuf>).unwrap();
///
/// // Use explicit path
/// let result = discover_config(Some("custom.toml")).unwrap();
/// ```
pub fn discover_config<P: AsRef<Path>>(explicit_path: Option<P>) -> Result<DiscoveryResult> {
    // 1. If explicit path is provided, use it directly
    if let Some(path) = explicit_path {
        let path = path.as_ref();
        if path.exists() {
            return Ok(DiscoveryResult::Found(path.to_path_buf()));
        } else {
            anyhow::bail!("Configuration file not found at specified path: {}", path.display());
        }
    }

    // 2. Search in current directory
    if let Some(path) = search_current_dir()? {
        return Ok(DiscoveryResult::Found(path));
    }

    // 3. Search in parent directories
    if let Some(path) = search_parent_dirs()? {
        return Ok(DiscoveryResult::Found(path));
    }

    // 4. Search in XDG config directory
    if let Some(path) = search_xdg_config()? {
        return Ok(DiscoveryResult::Found(path));
    }

    // 5. Search in home directory
    if let Some(path) = search_home_dir()? {
        return Ok(DiscoveryResult::Found(path));
    }

    // Not found anywhere
    Ok(DiscoveryResult::NotFound)
}

/// Search for configuration file in current directory
pub fn search_current_dir() -> Result<Option<PathBuf>> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let config_path = current_dir.join(CONFIG_FILENAME);

    if config_path.exists() {
        tracing::debug!("Found config in current directory: {}", config_path.display());
        Ok(Some(config_path))
    } else {
        Ok(None)
    }
}

/// Search for configuration file in parent directories
///
/// Walks up the directory tree from the current directory until
/// a configuration file is found or the root is reached.
pub fn search_parent_dirs() -> Result<Option<PathBuf>> {
    let mut current = env::current_dir().context("Failed to get current directory")?;

    while let Some(parent) = current.parent() {
        let config_path = parent.join(CONFIG_FILENAME);
        if config_path.exists() {
            tracing::debug!("Found config in parent directory: {}", config_path.display());
            return Ok(Some(config_path));
        }
        current = parent.to_path_buf();
    }

    Ok(None)
}

/// Search for configuration file in XDG Base Directory
///
/// Looks for: `~/.config/octofhir/codegen.toml`
/// or uses $XDG_CONFIG_HOME if set
pub fn search_xdg_config() -> Result<Option<PathBuf>> {
    let config_dir = if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config_home)
    } else if let Some(home) = get_home_dir() {
        home.join(".config")
    } else {
        return Ok(None);
    };

    let config_path = config_dir.join(XDG_CONFIG_DIR).join(CONFIG_FILENAME);
    if config_path.exists() {
        tracing::debug!("Found config in XDG directory: {}", config_path.display());
        Ok(Some(config_path))
    } else {
        Ok(None)
    }
}

/// Search for configuration file in home directory
///
/// Looks for: `~/.octofhir/codegen.toml`
pub fn search_home_dir() -> Result<Option<PathBuf>> {
    if let Some(home) = get_home_dir() {
        let config_path = home.join(".octofhir").join(CONFIG_FILENAME);
        if config_path.exists() {
            tracing::debug!("Found config in home directory: {}", config_path.display());
            return Ok(Some(config_path));
        }
    }
    Ok(None)
}

/// Get the user's home directory
fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from).or_else(dirs::home_dir)
}

/// Generate a helpful error message when config is not found
pub fn not_found_error_message() -> String {
    let current_dir = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<unknown>".to_string());

    let xdg_path = if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_home).join(XDG_CONFIG_DIR).join(CONFIG_FILENAME)
    } else if let Some(home) = get_home_dir() {
        home.join(".config").join(XDG_CONFIG_DIR).join(CONFIG_FILENAME)
    } else {
        PathBuf::from("~/.config/octofhir/codegen.toml")
    };

    let home_path = if let Some(home) = get_home_dir() {
        home.join(".octofhir").join(CONFIG_FILENAME)
    } else {
        PathBuf::from("~/.octofhir/codegen.toml")
    };

    format!(
        r#"Configuration file '{}' not found.

Searched in:
  • Current directory: {}
  • Parent directories (walking up from current)
  • XDG config: {}
  • Home directory: {}

To create a new configuration file, run:
  octofhir-codegen init

Or specify a custom path:
  octofhir-codegen --config path/to/config.toml <command>
"#,
        CONFIG_FILENAME,
        current_dir,
        xdg_path.display(),
        home_path.display()
    )
}

/// Ensure a configuration file exists or return a helpful error
pub fn ensure_config_exists<P: AsRef<Path>>(explicit_path: Option<P>) -> Result<PathBuf> {
    let result = discover_config(explicit_path)?;
    result.ok_or_else(|| anyhow::anyhow!("{}", not_found_error_message()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_discovery_result_is_found() {
        let found = DiscoveryResult::Found(PathBuf::from("/tmp/config.toml"));
        assert!(found.is_found());

        let not_found = DiscoveryResult::NotFound;
        assert!(!not_found.is_found());
    }

    #[test]
    fn test_discovery_result_path() {
        let path = PathBuf::from("/tmp/config.toml");
        let found = DiscoveryResult::Found(path.clone());
        assert_eq!(found.path(), Some(path.as_path()));

        let not_found = DiscoveryResult::NotFound;
        assert_eq!(not_found.path(), None);
    }

    #[test]
    fn test_discovery_result_ok_or_else() {
        let path = PathBuf::from("/tmp/config.toml");
        let found = DiscoveryResult::Found(path.clone());
        assert_eq!(found.ok_or_else(|| "error").unwrap(), path);

        let not_found = DiscoveryResult::NotFound;
        assert_eq!(not_found.ok_or_else(|| "error").unwrap_err(), "error");
    }

    #[test]
    fn test_search_current_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(CONFIG_FILENAME);
        fs::write(&config_path, "test").unwrap();

        // Change to temp directory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let result = search_current_dir().unwrap();
        assert!(result.is_some());
        // Canonicalize both paths to handle symlinks on macOS
        let result_canonical = result.unwrap().canonicalize().unwrap();
        let expected_canonical = config_path.canonicalize().unwrap();
        assert_eq!(result_canonical, expected_canonical);

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_search_current_dir_not_found() {
        let temp_dir = TempDir::new().unwrap();

        // Change to temp directory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let result = search_current_dir().unwrap();
        assert!(result.is_none());

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_search_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(CONFIG_FILENAME);
        fs::write(&config_path, "test").unwrap();

        // Create subdirectory and change to it
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&subdir).unwrap();

        let result = search_parent_dirs().unwrap();
        assert!(result.is_some());
        // Canonicalize both paths to handle symlinks on macOS
        let result_canonical = result.unwrap().canonicalize().unwrap();
        let expected_canonical = config_path.canonicalize().unwrap();
        assert_eq!(result_canonical, expected_canonical);

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_discover_config_with_explicit_path() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("custom.toml");
        fs::write(&config_path, "test").unwrap();

        let result = discover_config(Some(&config_path)).unwrap();
        assert!(result.is_found());
        assert_eq!(result.path().unwrap(), config_path);
    }

    #[test]
    fn test_discover_config_explicit_path_not_found() {
        let result = discover_config(Some("/nonexistent/path.toml"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found at specified path"));
    }

    #[test]
    fn test_not_found_error_message() {
        let message = not_found_error_message();
        assert!(message.contains(CONFIG_FILENAME));
        assert!(message.contains("Searched in:"));
        assert!(message.contains("octofhir-codegen init"));
    }

    #[test]
    fn test_ensure_config_exists_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let result = ensure_config_exists(None::<PathBuf>);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_ensure_config_exists_found() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(CONFIG_FILENAME);
        fs::write(&config_path, "test").unwrap();

        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let result = ensure_config_exists(None::<PathBuf>).unwrap();
        // Canonicalize both paths to handle symlinks on macOS
        let result_canonical = result.canonicalize().unwrap();
        let expected_canonical = config_path.canonicalize().unwrap();
        assert_eq!(result_canonical, expected_canonical);

        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_get_home_dir() {
        let home = get_home_dir();
        // Should return Some on most systems
        assert!(home.is_some() || cfg!(target_os = "unknown"));
    }
}
