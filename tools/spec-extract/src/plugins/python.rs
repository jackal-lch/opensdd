use anyhow::Result;
use std::path::Path;
use tree_sitter::Node;

use crate::parser::{LanguagePlugin, NodeHelper, TreeSitterParser};
use crate::spec::{ConstSpec, FileSpec, FuncSpec, TypeSpec};

/// Plugin for extracting specifications from Python source files.
pub struct PythonPlugin;

impl PythonPlugin {
    pub fn new() -> Self {
        Self
    }

    fn extract_module(&self, path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    fn extract_imports(&self, root: Node, source: &str) -> Vec<String> {
        let mut imports = Vec::new();

        // import x, y, z
        for import_stmt in NodeHelper::find_all(root, "import_statement") {
            let text = NodeHelper::text(import_stmt, source);
            if let Some(module) = text.strip_prefix("import ") {
                imports.push(module.trim().to_string());
            }
        }

        // from x import y
        for import_from in NodeHelper::find_all(root, "import_from_statement") {
            let text = NodeHelper::text(import_from, source);
            imports.push(text.to_string());
        }

        imports
    }

    fn extract_classes(&self, root: Node, source: &str) -> Vec<TypeSpec> {
        let mut types = Vec::new();

        for class_def in NodeHelper::find_all(root, "class_definition") {
            if let Some(spec) = self.extract_class(class_def, source) {
                types.push(spec);
            }
        }

        types
    }

    fn extract_class(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        // Extract docstring as documentation
        let doc = self.extract_docstring(node, source);

        // Extract base classes
        let mut embeds = Vec::new();
        if let Some(bases) = NodeHelper::child_by_kind(node, "argument_list") {
            let mut cursor = bases.walk();
            for child in bases.children(&mut cursor) {
                if child.kind() == "identifier" || child.kind() == "attribute" {
                    embeds.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract fields from __init__ method type hints and class attributes
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        if let Some(body) = NodeHelper::child_by_kind(node, "block") {
            // Look for class-level typed assignments
            for assign in NodeHelper::find_all(body, "typed_assignment") {
                let name_node = assign.child(0);
                let type_node = assign.child(2);

                if let (Some(n), Some(t)) = (name_node, type_node) {
                    let field_name = NodeHelper::text(n, source);
                    let field_type = NodeHelper::text(t, source);
                    // Skip if it looks like a method (has parentheses in type)
                    if !field_type.contains("(") {
                        fields.push(format!("{}: {}", field_name, field_type));
                    }
                }
            }

            // Look for method definitions
            for func_def in NodeHelper::children_by_kind(body, "function_definition") {
                let method_name = NodeHelper::field(func_def, "name")
                    .map(|n| NodeHelper::text(n, source));

                if let Some(name) = method_name {
                    // Skip dunder methods for method list, but use them for analysis
                    if !name.starts_with("__") || name == "__init__" {
                        if let Some(sig) = self.build_func_signature(func_def, source) {
                            methods.push(sig);
                        }
                    }

                    // Extract field hints from __init__
                    if name == "__init__" {
                        for assign in NodeHelper::find_all(func_def, "assignment") {
                            // Look for self.x = ...
                            if let Some(left) = assign.child(0) {
                                if left.kind() == "attribute" {
                                    let attr_text = NodeHelper::text(left, source);
                                    if attr_text.starts_with("self.") {
                                        let field_name = &attr_text[5..];
                                        // Only add if not already in fields
                                        if !fields.iter().any(|f| f.starts_with(field_name)) {
                                            fields.push(field_name.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Some(TypeSpec {
            name: format!("{} class", name),
            doc,
            kind: "class".to_string(),
            fields,
            methods,
            embeds,
            ..Default::default()
        })
    }

    fn extract_docstring(&self, node: Node, source: &str) -> Option<String> {
        // Look for the body/block of the class/function
        let body = NodeHelper::child_by_kind(node, "block")?;

        // The first expression statement with a string might be a docstring
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            if child.kind() == "expression_statement" {
                if let Some(string_node) = child.child(0) {
                    if string_node.kind() == "string" {
                        let text = NodeHelper::text(string_node, source);
                        // Clean up triple quotes
                        let cleaned = text
                            .trim_start_matches("\"\"\"")
                            .trim_start_matches("'''")
                            .trim_end_matches("\"\"\"")
                            .trim_end_matches("'''")
                            .trim()
                            .to_string();
                        return Some(cleaned);
                    }
                }
            }
            // Stop looking after we hit a non-expression statement
            break;
        }

        None
    }

    fn extract_functions(&self, root: Node, source: &str) -> Vec<FuncSpec> {
        let mut functions = Vec::new();

        for func_def in NodeHelper::find_all(root, "function_definition") {
            // Skip functions inside classes (those are methods)
            if self.is_inside_class(func_def) {
                continue;
            }

            if let Some(spec) = self.extract_func_spec(func_def, source) {
                functions.push(spec);
            }
        }

        functions
    }

    fn is_inside_class(&self, node: Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "class_definition" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn extract_func_spec(&self, node: Node, source: &str) -> Option<FuncSpec> {
        let signature = self.build_func_signature(node, source)?;
        let doc = self.extract_docstring(node, source);

        // Check for decorators
        let has_self = self.has_self_param(node, source);

        Some(FuncSpec {
            signature,
            doc,
            receiver: if has_self { Some("self".to_string()) } else { None },
            ..Default::default()
        })
    }

    fn build_func_signature(&self, node: Node, source: &str) -> Option<String> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(node, "parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("()");

        let return_type = NodeHelper::field(node, "return_type")
            .map(|n| format!(" -> {}", NodeHelper::text(n, source)))
            .unwrap_or_default();

        Some(format!("def {}{}{}", name, params, return_type))
    }

    fn has_self_param(&self, node: Node, source: &str) -> bool {
        if let Some(params) = NodeHelper::field(node, "parameters") {
            let text = NodeHelper::text(params, source);
            // Check if first param is self or cls
            text.starts_with("(self") || text.starts_with("(cls")
        } else {
            false
        }
    }

    fn extract_constants(&self, root: Node, source: &str) -> Vec<ConstSpec> {
        let mut constants = Vec::new();

        // Look for module-level assignments that look like constants (UPPER_CASE)
        for node in NodeHelper::find_all(root, "expression_statement") {
            // Skip if inside a class or function
            if self.is_inside_class(node) || self.is_inside_function(node) {
                continue;
            }

            if let Some(assign) = NodeHelper::child_by_kind(node, "assignment") {
                let left = assign.child(0);
                let right = assign.child(2);

                if let Some(left) = left {
                    if left.kind() == "identifier" {
                        let name = NodeHelper::text(left, source);
                        // Check if it looks like a constant (all uppercase)
                        if name.chars().all(|c| c.is_uppercase() || c == '_') {
                            let value = right.map(|r| NodeHelper::text(r, source).to_string());
                            constants.push(ConstSpec {
                                name: name.to_string(),
                                type_name: None,
                                value,
                                doc: None,
                            });
                        }
                    }
                }
            }
        }

        constants
    }

    fn is_inside_function(&self, node: Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "function_definition" {
                return true;
            }
            current = parent.parent();
        }
        false
    }
}

impl Default for PythonPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePlugin for PythonPlugin {
    fn name(&self) -> &'static str {
        "python"
    }

    fn extensions(&self) -> &[&'static str] {
        &["py"]
    }

    fn extract(&self, source: &str, path: &Path) -> Result<FileSpec> {
        let language = tree_sitter_python::LANGUAGE;
        let mut parser = TreeSitterParser::new(language.into())?;
        let tree = parser.parse(source)?;
        let root = tree.root_node();

        Ok(FileSpec {
            file: path.to_string_lossy().to_string(),
            package: self.extract_module(path),
            imports: self.extract_imports(root, source),
            types: self.extract_classes(root, source),
            functions: self.extract_functions(root, source),
            methods: Vec::new(), // Python methods are included in class types
            constants: self.extract_constants(root, source),
            variables: Vec::new(),
            errors: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_class() {
        let plugin = PythonPlugin::new();
        let source = r#"
class User:
    """Represents a user in the system."""

    def __init__(self, name: str):
        self.name = name

    def greet(self) -> str:
        return f"Hello, {self.name}"
"#;
        let spec = plugin.extract(source, Path::new("user.py")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "User class");
        assert!(spec.types[0].doc.as_ref().unwrap().contains("user in the system"));
    }

    #[test]
    fn test_extract_function() {
        let plugin = PythonPlugin::new();
        let source = r#"
def hello(name: str) -> str:
    """Return a greeting."""
    return f"Hello, {name}"
"#;
        let spec = plugin.extract(source, Path::new("hello.py")).unwrap();
        assert_eq!(spec.functions.len(), 1);
        assert!(spec.functions[0].signature.contains("hello"));
    }

    #[test]
    fn test_extract_class_with_inheritance() {
        let plugin = PythonPlugin::new();
        let source = r#"
class Admin(User, Permissioned):
    """An admin user."""
    pass
"#;
        let spec = plugin.extract(source, Path::new("admin.py")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert!(spec.types[0].embeds.contains(&"User".to_string()));
    }

    #[test]
    fn test_extract_constants() {
        let plugin = PythonPlugin::new();
        let source = r#"
MAX_RETRIES = 3
DEFAULT_TIMEOUT = 30
"#;
        let spec = plugin.extract(source, Path::new("config.py")).unwrap();
        assert_eq!(spec.constants.len(), 2);
    }
}
