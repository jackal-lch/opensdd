pub mod output;
pub mod parser;
pub mod plugins;
pub mod spec;

pub use output::{extract_spec, generate_file_spec, IndexBuilder};
pub use parser::{ExtractOptions, LanguagePlugin};
pub use plugins::PluginRegistry;
pub use spec::{write_extracted_spec, ExtractedSpec, FileSpec, OutputFormat};
