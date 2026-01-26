use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::plugins::PluginRegistry;
use crate::spec::{write_file_spec, FileSpec, OutputFormat};

/// Generate a spec file for a single source file.
pub fn generate_file_spec(
    source_path: &Path,
    output_dir: &Path,
    format: OutputFormat,
    registry: &PluginRegistry,
) -> Result<Option<PathBuf>> {
    // Get the appropriate plugin
    let plugin = match registry.get_for_file(source_path) {
        Some(p) => p,
        None => return Ok(None),
    };

    // Read the source file
    let source = fs::read_to_string(source_path)
        .with_context(|| format!("Failed to read source file: {}", source_path.display()))?;

    // Extract the spec
    let spec = plugin
        .extract(&source, source_path)
        .with_context(|| format!("Failed to extract spec from: {}", source_path.display()))?;

    // Determine output path
    let output_path = get_output_path(source_path, output_dir, format);

    // Write the spec
    write_file_spec(&spec, &output_path, format)?;

    Ok(Some(output_path))
}

/// Generate spec from source content without writing to file.
pub fn extract_spec(
    source: &str,
    source_path: &Path,
    registry: &PluginRegistry,
) -> Result<Option<FileSpec>> {
    let plugin = match registry.get_for_file(source_path) {
        Some(p) => p,
        None => return Ok(None),
    };

    let spec = plugin
        .extract(source, source_path)
        .with_context(|| format!("Failed to extract spec from: {}", source_path.display()))?;

    Ok(Some(spec))
}

/// Calculate the output path for a source file.
fn get_output_path(source_path: &Path, output_dir: &Path, format: OutputFormat) -> PathBuf {
    let file_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let ext = source_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let spec_name = format!("{}.{}.{}", file_name, ext, format.extension());
    output_dir.join(spec_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_path() {
        let source = Path::new("/src/main.go");
        let output = Path::new("/specs");

        let result = get_output_path(source, output, OutputFormat::Yaml);
        assert_eq!(result, PathBuf::from("/specs/main.go.yaml"));

        let result = get_output_path(source, output, OutputFormat::Json);
        assert_eq!(result, PathBuf::from("/specs/main.go.json"));
    }
}
