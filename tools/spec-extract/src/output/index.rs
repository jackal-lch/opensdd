use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::spec::{write_index_spec, FileEntry, FileSpec, IndexSpec, IndexStats, OutputFormat};

/// Builder for creating an index of all extracted specs.
pub struct IndexBuilder {
    project: String,
    root: PathBuf,
    entries: Vec<(FileEntry, FileSpec)>,
}

impl IndexBuilder {
    pub fn new(project: impl Into<String>, root: impl Into<PathBuf>) -> Self {
        Self {
            project: project.into(),
            root: root.into(),
            entries: Vec::new(),
        }
    }

    /// Add a file entry to the index.
    pub fn add(&mut self, spec_path: PathBuf, source_path: PathBuf, language: String, spec: FileSpec) {
        let entry = FileEntry {
            spec: spec_path.to_string_lossy().to_string(),
            source: source_path.to_string_lossy().to_string(),
            language,
        };
        self.entries.push((entry, spec));
    }

    /// Build the index spec.
    pub fn build(self) -> IndexSpec {
        let mut total_types = 0;
        let mut total_functions = 0;
        let mut total_methods = 0;
        let mut by_language: HashMap<String, usize> = HashMap::new();

        let files: Vec<FileEntry> = self
            .entries
            .into_iter()
            .map(|(entry, spec)| {
                total_types += spec.types.len();
                total_functions += spec.functions.len();
                total_methods += spec.methods.len();

                *by_language.entry(entry.language.clone()).or_insert(0) += 1;

                entry
            })
            .collect();

        let stats = IndexStats {
            total_files: files.len(),
            total_types,
            total_functions,
            total_methods,
            by_language,
        };

        IndexSpec {
            project: self.project,
            root: self.root.to_string_lossy().to_string(),
            files,
            stats: Some(stats),
        }
    }

    /// Build and write the index to a file.
    pub fn write(self, path: &Path, format: OutputFormat) -> Result<()> {
        let index = self.build();
        write_index_spec(&index, path, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_builder() {
        let mut builder = IndexBuilder::new("test-project", "/src");

        builder.add(
            PathBuf::from("specs/main.go.yaml"),
            PathBuf::from("src/main.go"),
            "go".to_string(),
            FileSpec {
                file: "src/main.go".to_string(),
                package: "main".to_string(),
                types: vec![],
                functions: vec![Default::default()],
                ..Default::default()
            },
        );

        builder.add(
            PathBuf::from("specs/lib.rs.yaml"),
            PathBuf::from("src/lib.rs"),
            "rust".to_string(),
            FileSpec {
                file: "src/lib.rs".to_string(),
                package: "lib".to_string(),
                types: vec![Default::default()],
                functions: vec![],
                ..Default::default()
            },
        );

        let index = builder.build();

        assert_eq!(index.project, "test-project");
        assert_eq!(index.files.len(), 2);

        let stats = index.stats.unwrap();
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_types, 1);
        assert_eq!(stats.total_functions, 1);
        assert_eq!(*stats.by_language.get("go").unwrap(), 1);
        assert_eq!(*stats.by_language.get("rust").unwrap(), 1);
    }
}
