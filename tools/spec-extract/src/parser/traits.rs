use anyhow::Result;
use std::path::Path;

use crate::spec::FileSpec;

/// Trait that all language plugins must implement.
///
/// Each plugin is responsible for extracting specifications from source files
/// of a particular language using tree-sitter parsing.
pub trait LanguagePlugin: Send + Sync {
    /// Returns the plugin identifier (e.g., "go", "rust", "python", "typescript").
    fn name(&self) -> &'static str;

    /// Returns the file extensions this plugin handles (e.g., &["go"] for Go).
    fn extensions(&self) -> &[&'static str];

    /// Parse source code and extract a FileSpec.
    ///
    /// # Arguments
    /// * `source` - The source code content
    /// * `path` - The path to the source file (for context/naming)
    ///
    /// # Returns
    /// A FileSpec containing all extracted specifications, or an error.
    fn extract(&self, source: &str, path: &Path) -> Result<FileSpec>;

    /// Optional: Perform deeper semantic analysis on an already-extracted spec.
    ///
    /// This can be used for things like:
    /// - Resolving type references
    /// - Detecting interface implementations
    /// - Analyzing control flow
    ///
    /// Default implementation does nothing.
    fn analyze_semantics(&self, _spec: &mut FileSpec, _source: &str) -> Result<()> {
        Ok(())
    }

    /// Check if this plugin can handle the given file path.
    fn can_handle(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.extensions().contains(&ext))
            .unwrap_or(false)
    }
}

/// Options for extraction behavior.
#[derive(Debug, Clone, Default)]
pub struct ExtractOptions {
    /// Whether to extract behavior specifications from doc comments.
    pub extract_behavior: bool,
    /// Whether to include private/unexported items.
    pub include_private: bool,
    /// Whether to resolve type references.
    pub resolve_types: bool,
}
