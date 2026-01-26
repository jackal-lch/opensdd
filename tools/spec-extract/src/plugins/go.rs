use anyhow::Result;
use std::path::Path;
use tree_sitter::Node;

use crate::parser::{LanguagePlugin, NodeHelper, TreeSitterParser};
use crate::spec::{ConstSpec, ErrorSpec, FileSpec, FuncSpec, TypeSpec, VarSpec};

/// Plugin for extracting specifications from Go source files.
pub struct GoPlugin;

impl GoPlugin {
    pub fn new() -> Self {
        Self
    }

    fn extract_package(&self, root: Node, source: &str) -> String {
        NodeHelper::find_all(root, "package_clause")
            .first()
            .and_then(|node| {
                // Try field "name" first, then look for package_identifier child
                NodeHelper::field(*node, "name")
                    .or_else(|| NodeHelper::child_by_kind(*node, "package_identifier"))
            })
            .map(|n| NodeHelper::text(n, source).to_string())
            .unwrap_or_default()
    }

    fn extract_imports(&self, root: Node, source: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for import_decl in NodeHelper::find_all(root, "import_declaration") {
            // Handle single import
            if let Some(spec) = NodeHelper::child_by_kind(import_decl, "import_spec") {
                if let Some(path) = NodeHelper::field(spec, "path") {
                    let import_path = NodeHelper::text(path, source)
                        .trim_matches('"')
                        .to_string();
                    imports.push(import_path);
                }
            }

            // Handle import group
            if let Some(spec_list) = NodeHelper::child_by_kind(import_decl, "import_spec_list") {
                for spec in NodeHelper::children_by_kind(spec_list, "import_spec") {
                    if let Some(path) = NodeHelper::field(spec, "path") {
                        let import_path = NodeHelper::text(path, source)
                            .trim_matches('"')
                            .to_string();
                        imports.push(import_path);
                    }
                }
            }
        }

        imports
    }

    fn extract_types(&self, root: Node, source: &str) -> Vec<TypeSpec> {
        let mut types = Vec::new();

        for type_decl in NodeHelper::find_all(root, "type_declaration") {
            for type_spec in NodeHelper::children_by_kind(type_decl, "type_spec") {
                if let Some(spec) = self.extract_type_spec(type_spec, type_decl, source) {
                    types.push(spec);
                }
            }
        }

        types
    }

    fn extract_type_spec(&self, type_spec: Node, type_decl: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(type_spec, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let type_node = NodeHelper::field(type_spec, "type")?;
        let kind = type_node.kind();

        let doc = NodeHelper::preceding_comment(type_decl, source);

        match kind {
            "struct_type" => Some(self.extract_struct(name, type_node, doc, source)),
            "interface_type" => Some(self.extract_interface(name, type_node, doc, source)),
            _ => {
                // Type alias
                Some(TypeSpec {
                    name: format!("{} type", name),
                    doc,
                    kind: "type_alias".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    fn extract_struct(&self, name: String, node: Node, doc: Option<String>, source: &str) -> TypeSpec {
        let mut fields = Vec::new();
        let mut embeds = Vec::new();

        if let Some(field_list) = NodeHelper::child_by_kind(node, "field_declaration_list") {
            let mut cursor = field_list.walk();
            for field in field_list.children(&mut cursor) {
                if field.kind() == "field_declaration" {
                    // Check for embedded field (no name, just type)
                    let name_node = NodeHelper::field(field, "name");
                    let type_node = NodeHelper::field(field, "type");

                    if name_node.is_none() {
                        // Embedded type
                        if let Some(t) = type_node {
                            embeds.push(NodeHelper::text(t, source).to_string());
                        }
                    } else if let (Some(n), Some(t)) = (name_node, type_node) {
                        let field_name = NodeHelper::text(n, source);
                        let field_type = NodeHelper::text(t, source);
                        fields.push(format!("{}: {}", field_name, field_type));
                    }
                }
            }
        }

        TypeSpec {
            name: format!("{} struct", name),
            doc,
            kind: "struct".to_string(),
            fields,
            embeds,
            ..Default::default()
        }
    }

    fn extract_interface(&self, name: String, node: Node, doc: Option<String>, source: &str) -> TypeSpec {
        let mut methods = Vec::new();
        let mut embeds = Vec::new();

        // The interface_type may contain method_spec, method_elem, or embedded types
        // Different tree-sitter-go versions use different node names
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "method_spec" | "method_elem" => {
                    let method_name = NodeHelper::field(child, "name")
                        .map(|n| NodeHelper::text(n, source))
                        .unwrap_or("");
                    let params = NodeHelper::field(child, "parameters")
                        .or_else(|| NodeHelper::child_by_kind(child, "parameter_list"))
                        .map(|n| NodeHelper::text(n, source))
                        .unwrap_or("()");
                    let result = NodeHelper::field(child, "result")
                        .or_else(|| NodeHelper::child_by_kind(child, "result"))
                        .map(|n| format!(" {}", NodeHelper::text(n, source)))
                        .unwrap_or_default();
                    methods.push(format!("{}{}{}", method_name, params, result));
                }
                "type_identifier" | "qualified_type" | "constraint_elem" => {
                    embeds.push(NodeHelper::text(child, source).to_string());
                }
                _ => {}
            }
        }

        TypeSpec {
            name: format!("{} interface", name),
            doc,
            kind: "interface".to_string(),
            methods,
            embeds,
            ..Default::default()
        }
    }

    fn extract_functions(&self, root: Node, source: &str) -> Vec<FuncSpec> {
        let mut functions = Vec::new();

        for func_decl in NodeHelper::find_all(root, "function_declaration") {
            if let Some(spec) = self.extract_func_spec(func_decl, source) {
                functions.push(spec);
            }
        }

        functions
    }

    fn extract_methods(&self, root: Node, source: &str) -> Vec<FuncSpec> {
        let mut methods = Vec::new();

        for method_decl in NodeHelper::find_all(root, "method_declaration") {
            if let Some(spec) = self.extract_method_spec(method_decl, source) {
                methods.push(spec);
            }
        }

        methods
    }

    fn extract_func_spec(&self, node: Node, source: &str) -> Option<FuncSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(node, "parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("()");

        let result = NodeHelper::field(node, "result")
            .map(|n| format!(" {}", NodeHelper::text(n, source)))
            .unwrap_or_default();

        let type_params = NodeHelper::field(node, "type_parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or_default();

        let signature = format!("func {}{}{}{}", name, type_params, params, result);
        let doc = NodeHelper::preceding_comment(node, source);

        Some(FuncSpec {
            signature,
            doc,
            ..Default::default()
        })
    }

    fn extract_method_spec(&self, node: Node, source: &str) -> Option<FuncSpec> {
        let receiver = NodeHelper::field(node, "receiver")
            .map(|n| NodeHelper::text(n, source).to_string());

        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(node, "parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("()");

        let result = NodeHelper::field(node, "result")
            .map(|n| format!(" {}", NodeHelper::text(n, source)))
            .unwrap_or_default();

        let receiver_str = receiver.as_deref().unwrap_or("()");
        let signature = format!("func {} {}{}{}", receiver_str, name, params, result);
        let doc = NodeHelper::preceding_comment(node, source);

        Some(FuncSpec {
            signature,
            doc,
            receiver,
            ..Default::default()
        })
    }

    fn extract_constants(&self, root: Node, source: &str) -> Vec<ConstSpec> {
        let mut constants = Vec::new();

        for const_decl in NodeHelper::find_all(root, "const_declaration") {
            let doc = NodeHelper::preceding_comment(const_decl, source);

            for spec in NodeHelper::children_by_kind(const_decl, "const_spec") {
                let name = NodeHelper::field(spec, "name")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let type_name = NodeHelper::field(spec, "type")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let value = NodeHelper::field(spec, "value")
                    .map(|n| NodeHelper::text(n, source).to_string());

                if let Some(name) = name {
                    constants.push(ConstSpec {
                        name,
                        type_name,
                        value,
                        doc: doc.clone(),
                    });
                }
            }
        }

        constants
    }

    fn extract_variables(&self, root: Node, source: &str) -> Vec<VarSpec> {
        let mut variables = Vec::new();

        for var_decl in NodeHelper::find_all(root, "var_declaration") {
            let doc = NodeHelper::preceding_comment(var_decl, source);

            for spec in NodeHelper::children_by_kind(var_decl, "var_spec") {
                let name = NodeHelper::field(spec, "name")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let type_name = NodeHelper::field(spec, "type")
                    .map(|n| NodeHelper::text(n, source).to_string());

                if let Some(name) = name {
                    variables.push(VarSpec {
                        name,
                        type_name,
                        doc: doc.clone(),
                    });
                }
            }
        }

        variables
    }

    fn extract_errors(&self, root: Node, source: &str) -> Vec<ErrorSpec> {
        let mut errors = Vec::new();

        // Look for error variables (var ErrXxx = errors.New(...))
        for var_decl in NodeHelper::find_all(root, "var_declaration") {
            for spec in NodeHelper::children_by_kind(var_decl, "var_spec") {
                let name = NodeHelper::field(spec, "name")
                    .map(|n| NodeHelper::text(n, source));

                if let Some(name) = name {
                    if name.starts_with("Err") || name.starts_with("err") {
                        let doc = NodeHelper::preceding_comment(var_decl, source);
                        let message = NodeHelper::field(spec, "value")
                            .map(|n| NodeHelper::text(n, source).to_string());

                        errors.push(ErrorSpec {
                            name: name.to_string(),
                            message,
                            doc,
                        });
                    }
                }
            }
        }

        errors
    }
}

impl Default for GoPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePlugin for GoPlugin {
    fn name(&self) -> &'static str {
        "go"
    }

    fn extensions(&self) -> &[&'static str] {
        &["go"]
    }

    fn extract(&self, source: &str, path: &Path) -> Result<FileSpec> {
        let language = tree_sitter_go::LANGUAGE;
        let mut parser = TreeSitterParser::new(language.into())?;
        let tree = parser.parse(source)?;
        let root = tree.root_node();

        Ok(FileSpec {
            file: path.to_string_lossy().to_string(),
            package: self.extract_package(root, source),
            imports: self.extract_imports(root, source),
            types: self.extract_types(root, source),
            functions: self.extract_functions(root, source),
            methods: self.extract_methods(root, source),
            constants: self.extract_constants(root, source),
            variables: self.extract_variables(root, source),
            errors: self.extract_errors(root, source),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_package() {
        let plugin = GoPlugin::new();
        let source = "package main";
        let spec = plugin.extract(source, Path::new("test.go")).unwrap();
        assert_eq!(spec.package, "main");
    }

    #[test]
    fn test_extract_struct() {
        let plugin = GoPlugin::new();
        let source = r#"
package main

// User represents a system user.
type User struct {
    ID   int
    Name string
}
"#;
        let spec = plugin.extract(source, Path::new("test.go")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "User struct");
        assert_eq!(spec.types[0].kind, "struct");
        assert!(spec.types[0].doc.as_ref().unwrap().contains("system user"));
    }

    #[test]
    fn test_extract_interface() {
        let plugin = GoPlugin::new();
        let source = r#"
package io

type Reader interface {
    Read(p []byte) (n int, err error)
}
"#;
        let spec = plugin.extract(source, Path::new("io.go")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "Reader interface");
        assert_eq!(spec.types[0].kind, "interface");
        assert!(!spec.types[0].methods.is_empty());
    }

    #[test]
    fn test_extract_function() {
        let plugin = GoPlugin::new();
        let source = r#"
package main

// Hello returns a greeting.
func Hello(name string) string {
    return "Hello, " + name
}
"#;
        let spec = plugin.extract(source, Path::new("test.go")).unwrap();
        assert_eq!(spec.functions.len(), 1);
        assert!(spec.functions[0].signature.contains("Hello"));
    }

    #[test]
    fn test_extract_method() {
        let plugin = GoPlugin::new();
        let source = r#"
package main

type User struct {
    Name string
}

func (u *User) Greet() string {
    return "Hello, " + u.Name
}
"#;
        let spec = plugin.extract(source, Path::new("test.go")).unwrap();
        assert_eq!(spec.methods.len(), 1);
        assert!(spec.methods[0].receiver.is_some());
    }
}
