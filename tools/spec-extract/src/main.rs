use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use spec_extract::{
    generate_file_spec, write_extracted_spec, ExtractedSpec, IndexBuilder, OutputFormat,
    PluginRegistry,
};

#[derive(Parser)]
#[command(name = "spec-extract")]
#[command(about = "Extract code specifications from multiple languages into YAML/JSON format")]
#[command(version)]
struct Cli {
    /// Path to file or directory to extract specs from
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output path (directory for multi-file mode, file path for --single-file mode)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output format (yaml or json)
    #[arg(short, long, default_value = "yaml")]
    format: String,

    /// Filter by language (comma-separated: go,rust,python,typescript)
    #[arg(short, long)]
    lang: Option<String>,

    /// Extract behavior specifications from doc comments
    #[arg(long)]
    behavior: bool,

    /// Generate index.yaml file (ignored when --single-file is set)
    #[arg(long, default_value = "true")]
    index: bool,

    /// Output all specs into a single file instead of multiple files
    #[arg(long)]
    single_file: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let format = OutputFormat::from_str(&cli.format)
        .unwrap_or_else(|| {
            eprintln!("Warning: Unknown format '{}', defaulting to YAML", cli.format);
            OutputFormat::Yaml
        });

    let registry = PluginRegistry::new();

    // Parse language filter
    let lang_filter: Option<HashSet<String>> = cli.lang.map(|l| {
        l.split(',')
            .map(|s| s.trim().to_lowercase())
            .collect()
    });

    let path = &cli.path;

    if path.is_file() {
        // Single file mode (extracting one source file)
        let output_dir = cli.output.unwrap_or_else(|| PathBuf::from(".opensdd/extracted"));
        extract_single_file(path, &output_dir, format, &registry, cli.verbose)?;
    } else if path.is_dir() {
        // Directory mode
        if cli.single_file {
            // Single-file output: -o is a file path
            let output_file = cli.output.unwrap_or_else(|| {
                PathBuf::from(format!(".opensdd/extracted.{}", format.extension()))
            });
            extract_directory_single(path, &output_file, format, &registry, lang_filter, cli.verbose)?;
        } else {
            // Multi-file output: -o is a directory
            let output_dir = cli.output.unwrap_or_else(|| PathBuf::from(".opensdd/extracted"));
            extract_directory_multi(path, &output_dir, format, &registry, lang_filter, cli.verbose, cli.index)?;
        }
    } else {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    Ok(())
}

fn extract_single_file(
    path: &Path,
    output_dir: &Path,
    format: OutputFormat,
    registry: &PluginRegistry,
    verbose: bool,
) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    match generate_file_spec(path, output_dir, format, registry)? {
        Some(output_path) => {
            if verbose {
                println!("Extracted: {} -> {}", path.display(), output_path.display());
            } else {
                println!("{}", output_path.display());
            }
        }
        None => {
            eprintln!("Warning: No plugin found for file: {}", path.display());
        }
    }

    Ok(())
}

fn extract_directory_single(
    path: &Path,
    output_file: &Path,
    format: OutputFormat,
    registry: &PluginRegistry,
    lang_filter: Option<HashSet<String>>,
    verbose: bool,
) -> Result<()> {
    // Create parent directory if needed
    if let Some(parent) = output_file.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }

    let project_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project")
        .to_string();

    let mut extracted_count = 0;
    let mut skipped_count = 0;
    let mut all_specs: Vec<spec_extract::FileSpec> = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        if should_skip_path(file_path) {
            continue;
        }

        let plugin = match registry.get_for_file(file_path) {
            Some(p) => p,
            None => continue,
        };

        if let Some(ref filter) = lang_filter {
            if !filter.contains(plugin.name()) {
                continue;
            }
        }

        let source = fs::read_to_string(file_path)?;
        match spec_extract::extract_spec(&source, file_path, registry) {
            Ok(Some(spec)) => {
                if verbose {
                    println!("Extracted: {}", file_path.display());
                }
                all_specs.push(spec);
                extracted_count += 1;
            }
            Ok(None) => {
                skipped_count += 1;
            }
            Err(e) => {
                eprintln!("Error extracting {}: {}", file_path.display(), e);
                skipped_count += 1;
            }
        }
    }

    if extracted_count > 0 {
        let extracted_spec = ExtractedSpec {
            project: project_name,
            root: path.to_string_lossy().to_string(),
            extracted_at: Some(chrono::Utc::now().to_rfc3339()),
            files: all_specs,
        };

        write_extracted_spec(&extracted_spec, output_file, format)?;

        println!(
            "Extracted {} files ({} skipped) to {}",
            extracted_count,
            skipped_count,
            output_file.display()
        );
    } else {
        println!("No files extracted ({} skipped)", skipped_count);
    }

    Ok(())
}

fn extract_directory_multi(
    path: &Path,
    output_dir: &Path,
    format: OutputFormat,
    registry: &PluginRegistry,
    lang_filter: Option<HashSet<String>>,
    verbose: bool,
    generate_index: bool,
) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    let project_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project")
        .to_string();

    let mut index_builder = IndexBuilder::new(&project_name, path);
    let mut extracted_count = 0;
    let mut skipped_count = 0;

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        if should_skip_path(file_path) {
            continue;
        }

        let plugin = match registry.get_for_file(file_path) {
            Some(p) => p,
            None => continue,
        };

        if let Some(ref filter) = lang_filter {
            if !filter.contains(plugin.name()) {
                continue;
            }
        }

        let relative = file_path.strip_prefix(path).unwrap_or(file_path);
        let spec_output_dir = output_dir.join(relative.parent().unwrap_or(Path::new("")));

        match generate_file_spec(file_path, &spec_output_dir, format, registry) {
            Ok(Some(output_path)) => {
                if verbose {
                    println!("Extracted: {} -> {}", file_path.display(), output_path.display());
                }

                if generate_index {
                    let source = fs::read_to_string(file_path)?;
                    if let Ok(Some(spec)) = spec_extract::extract_spec(&source, file_path, registry) {
                        index_builder.add(
                            output_path,
                            file_path.to_path_buf(),
                            plugin.name().to_string(),
                            spec,
                        );
                    }
                }

                extracted_count += 1;
            }
            Ok(None) => {
                skipped_count += 1;
            }
            Err(e) => {
                eprintln!("Error extracting {}: {}", file_path.display(), e);
                skipped_count += 1;
            }
        }
    }

    if generate_index && extracted_count > 0 {
        let index_path = output_dir.join(format!("index.{}", format.extension()));
        index_builder.write(&index_path, format)?;

        if verbose {
            println!("Generated index: {}", index_path.display());
        }
    }

    println!(
        "Extracted {} files ({} skipped) to {}",
        extracted_count,
        skipped_count,
        output_dir.display()
    );

    Ok(())
}

fn should_skip_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Skip hidden files and directories
    for component in path.components() {
        if let std::path::Component::Normal(s) = component {
            if let Some(s) = s.to_str() {
                if s.starts_with('.') {
                    return true;
                }
            }
        }
    }

    // Skip common non-source directories
    let skip_patterns = [
        "node_modules",
        "vendor",
        "target",
        "dist",
        "build",
        "__pycache__",
        ".git",
        ".svn",
        ".hg",
    ];

    for pattern in skip_patterns {
        if path_str.contains(&format!("/{}/", pattern))
            || path_str.contains(&format!("\\{}\\", pattern))
        {
            return true;
        }
    }

    // Skip test files (optional, could be configurable)
    // For now, include test files

    false
}
