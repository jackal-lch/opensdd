use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::types::{ExtractedSpec, FileSpec, IndexSpec};

/// Output format for spec files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Yaml,
    Json,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Yaml => "yaml",
            OutputFormat::Json => "json",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "yaml" | "yml" => Some(OutputFormat::Yaml),
            "json" => Some(OutputFormat::Json),
            _ => None,
        }
    }
}

/// Write a FileSpec to a file in the specified format.
pub fn write_file_spec(spec: &FileSpec, path: &Path, format: OutputFormat) -> Result<()> {
    let content = match format {
        OutputFormat::Yaml => serde_yaml::to_string(spec)
            .context("Failed to serialize FileSpec to YAML")?,
        OutputFormat::Json => serde_json::to_string_pretty(spec)
            .context("Failed to serialize FileSpec to JSON")?,
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write spec to: {}", path.display()))?;

    Ok(())
}

/// Read a FileSpec from a file.
pub fn read_file_spec(path: &Path) -> Result<FileSpec> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read spec from: {}", path.display()))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "yaml" | "yml" => serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML from: {}", path.display())),
        "json" => serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from: {}", path.display())),
        _ => serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse spec from: {}", path.display())),
    }
}

/// Write an IndexSpec to a file in the specified format.
pub fn write_index_spec(spec: &IndexSpec, path: &Path, format: OutputFormat) -> Result<()> {
    let content = match format {
        OutputFormat::Yaml => serde_yaml::to_string(spec)
            .context("Failed to serialize IndexSpec to YAML")?,
        OutputFormat::Json => serde_json::to_string_pretty(spec)
            .context("Failed to serialize IndexSpec to JSON")?,
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write index to: {}", path.display()))?;

    Ok(())
}

/// Write an ExtractedSpec to a file in the specified format.
pub fn write_extracted_spec(spec: &ExtractedSpec, path: &Path, format: OutputFormat) -> Result<()> {
    let content = match format {
        OutputFormat::Yaml => serde_yaml::to_string(spec)
            .context("Failed to serialize ExtractedSpec to YAML")?,
        OutputFormat::Json => serde_json::to_string_pretty(spec)
            .context("Failed to serialize ExtractedSpec to JSON")?,
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write extracted spec to: {}", path.display()))?;

    Ok(())
}

/// Read an IndexSpec from a file.
pub fn read_index_spec(path: &Path) -> Result<IndexSpec> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read index from: {}", path.display()))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "yaml" | "yml" => serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML from: {}", path.display())),
        "json" => serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from: {}", path.display())),
        _ => serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse index from: {}", path.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_read_file_spec_yaml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.yaml");

        let spec = FileSpec {
            file: "test.go".to_string(),
            package: "main".to_string(),
            ..Default::default()
        };

        write_file_spec(&spec, &path, OutputFormat::Yaml).unwrap();
        let read_spec = read_file_spec(&path).unwrap();

        assert_eq!(spec.file, read_spec.file);
        assert_eq!(spec.package, read_spec.package);
    }

    #[test]
    fn test_write_read_file_spec_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");

        let spec = FileSpec {
            file: "test.rs".to_string(),
            package: "crate".to_string(),
            ..Default::default()
        };

        write_file_spec(&spec, &path, OutputFormat::Json).unwrap();
        let read_spec = read_file_spec(&path).unwrap();

        assert_eq!(spec.file, read_spec.file);
        assert_eq!(spec.package, read_spec.package);
    }
}
