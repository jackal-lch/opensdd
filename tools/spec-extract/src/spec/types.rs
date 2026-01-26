use serde::{Deserialize, Serialize};

/// Represents a complete specification for a single source file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileSpec {
    /// Path to the source file
    pub file: String,
    /// Package/module name
    pub package: String,
    /// Import statements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,
    /// Type definitions (structs, interfaces, classes, traits, enums)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<TypeSpec>,
    /// Standalone functions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<FuncSpec>,
    /// Methods (functions with receivers/self)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<FuncSpec>,
    /// Constants
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constants: Vec<ConstSpec>,
    /// Variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<VarSpec>,
    /// Error definitions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorSpec>,
}

/// Represents a type definition (struct, interface, class, trait, enum).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeSpec {
    /// Type name with kind suffix (e.g., "User struct", "Reader interface")
    pub name: String,
    /// Documentation comment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    /// Kind of type: struct, interface, class, trait, enum, type_alias
    pub kind: String,
    /// Fields (for structs/classes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,
    /// Method signatures (for interfaces/traits)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<String>,
    /// Embedded types (Go) or extended types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<String>,
    /// Interface/trait this type implements
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implements: Option<String>,
    /// Types that implement this interface/trait
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implemented_by: Vec<String>,
    /// Generic type parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_params: Vec<String>,
    /// Enum variants (for enums)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<String>,
}

/// Represents a function or method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuncSpec {
    /// Function signature
    pub signature: String,
    /// Documentation comment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    /// Receiver type (for methods)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
    /// Dependencies used by this function
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses: Vec<String>,
    /// Behavioral specification extracted from docs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Behavior>,
    /// Link to related tests
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tests: Option<TestLink>,
}

/// Behavioral specification extracted from documentation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Behavior {
    /// Preconditions that must hold before calling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions: Vec<String>,
    /// Postconditions guaranteed after calling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub postconditions: Vec<String>,
    /// Possible errors that can be returned
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    /// Side effects of calling this function
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
}

/// Link to related test functions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestLink {
    /// Test function name
    pub function: String,
    /// File containing the test
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

/// Represents a constant definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstSpec {
    /// Constant name
    pub name: String,
    /// Type of the constant
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Value (if simple literal)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Documentation comment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
}

/// Represents a variable definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VarSpec {
    /// Variable name
    pub name: String,
    /// Type of the variable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Documentation comment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
}

/// Represents an error definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorSpec {
    /// Error name or variable name
    pub name: String,
    /// Error message or description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Documentation comment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
}

/// Single-file extracted specification containing all files.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractedSpec {
    /// Project name
    pub project: String,
    /// Root directory that was scanned
    pub root: String,
    /// Timestamp of extraction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extracted_at: Option<String>,
    /// All extracted file specifications
    pub files: Vec<FileSpec>,
}

/// Index file containing references to all spec files in a project.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndexSpec {
    /// Project name
    pub project: String,
    /// Root directory
    pub root: String,
    /// List of spec files
    pub files: Vec<FileEntry>,
    /// Summary statistics
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<IndexStats>,
}

/// Entry for a single file in the index.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileEntry {
    /// Path to the spec file
    pub spec: String,
    /// Path to the source file
    pub source: String,
    /// Language of the source file
    pub language: String,
}

/// Statistics for the index.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_types: usize,
    pub total_functions: usize,
    pub total_methods: usize,
    pub by_language: std::collections::HashMap<String, usize>,
}
