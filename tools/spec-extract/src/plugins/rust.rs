use anyhow::Result;
use std::path::Path;
use tree_sitter::Node;

use crate::parser::{LanguagePlugin, NodeHelper, TreeSitterParser};
use crate::spec::{ConstSpec, FileSpec, FuncSpec, TypeSpec, VarSpec};

/// Plugin for extracting specifications from Rust source files.
pub struct RustPlugin;

impl RustPlugin {
    pub fn new() -> Self {
        Self
    }

    fn extract_module(&self, path: &Path) -> String {
        // Derive module name from file path
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    fn extract_uses(&self, root: Node, source: &str) -> Vec<String> {
        let mut uses = Vec::new();

        for use_decl in NodeHelper::find_all(root, "use_declaration") {
            if let Some(arg) = NodeHelper::child_by_kind(use_decl, "use_tree") {
                uses.push(NodeHelper::text(arg, source).to_string());
            } else if let Some(arg) = NodeHelper::child_by_kind(use_decl, "scoped_identifier") {
                uses.push(NodeHelper::text(arg, source).to_string());
            }
        }

        uses
    }

    fn extract_types(&self, root: Node, source: &str) -> Vec<TypeSpec> {
        let mut types = Vec::new();

        // Extract structs
        for struct_item in NodeHelper::find_all(root, "struct_item") {
            if let Some(spec) = self.extract_struct(struct_item, source) {
                types.push(spec);
            }
        }

        // Extract enums
        for enum_item in NodeHelper::find_all(root, "enum_item") {
            if let Some(spec) = self.extract_enum(enum_item, source) {
                types.push(spec);
            }
        }

        // Extract traits
        for trait_item in NodeHelper::find_all(root, "trait_item") {
            if let Some(spec) = self.extract_trait(trait_item, source) {
                types.push(spec);
            }
        }

        // Extract type aliases
        for type_item in NodeHelper::find_all(root, "type_item") {
            if let Some(spec) = self.extract_type_alias(type_item, source) {
                types.push(spec);
            }
        }

        types
    }

    fn extract_struct(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut fields = Vec::new();
        let mut type_params = Vec::new();

        // Extract type parameters
        if let Some(params) = NodeHelper::child_by_kind(node, "type_parameters") {
            let mut cursor = params.walk();
            for child in params.children(&mut cursor) {
                if child.kind() == "type_identifier" || child.kind() == "constrained_type_parameter" {
                    type_params.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract fields from field_declaration_list
        if let Some(field_list) = NodeHelper::child_by_kind(node, "field_declaration_list") {
            let mut cursor = field_list.walk();
            for field in field_list.children(&mut cursor) {
                if field.kind() == "field_declaration" {
                    let field_name = NodeHelper::field(field, "name")
                        .map(|n| NodeHelper::text(n, source));
                    let field_type = NodeHelper::field(field, "type")
                        .map(|n| NodeHelper::text(n, source));

                    if let (Some(n), Some(t)) = (field_name, field_type) {
                        fields.push(format!("{}: {}", n, t));
                    }
                }
            }
        }

        Some(TypeSpec {
            name: format!("{} struct", name),
            doc,
            kind: "struct".to_string(),
            fields,
            type_params,
            ..Default::default()
        })
    }

    fn extract_enum(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut variants = Vec::new();
        let mut type_params = Vec::new();

        // Extract type parameters
        if let Some(params) = NodeHelper::child_by_kind(node, "type_parameters") {
            let mut cursor = params.walk();
            for child in params.children(&mut cursor) {
                if child.kind() == "type_identifier" {
                    type_params.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract variants
        if let Some(body) = NodeHelper::child_by_kind(node, "enum_variant_list") {
            for variant in NodeHelper::children_by_kind(body, "enum_variant") {
                let variant_text = NodeHelper::text(variant, source);
                variants.push(variant_text.to_string());
            }
        }

        Some(TypeSpec {
            name: format!("{} enum", name),
            doc,
            kind: "enum".to_string(),
            variants,
            type_params,
            ..Default::default()
        })
    }

    fn extract_trait(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut methods = Vec::new();
        let mut type_params = Vec::new();

        // Extract type parameters
        if let Some(params) = NodeHelper::child_by_kind(node, "type_parameters") {
            let mut cursor = params.walk();
            for child in params.children(&mut cursor) {
                if child.kind() == "type_identifier" {
                    type_params.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract method signatures from declaration_list
        if let Some(body) = NodeHelper::child_by_kind(node, "declaration_list") {
            for func in NodeHelper::find_all(body, "function_signature_item") {
                if let Some(sig) = self.extract_func_signature(func, source) {
                    methods.push(sig);
                }
            }
        }

        Some(TypeSpec {
            name: format!("{} trait", name),
            doc,
            kind: "trait".to_string(),
            methods,
            type_params,
            ..Default::default()
        })
    }

    fn extract_type_alias(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        Some(TypeSpec {
            name: format!("{} type", name),
            doc,
            kind: "type_alias".to_string(),
            ..Default::default()
        })
    }

    fn extract_func_signature(&self, node: Node, source: &str) -> Option<String> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(node, "parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("()");

        let return_type = NodeHelper::child_by_kind(node, "return_type")
            .map(|n| format!(" -> {}", NodeHelper::text(n, source)))
            .unwrap_or_default();

        Some(format!("fn {}{}{}", name, params, return_type))
    }

    fn extract_functions(&self, root: Node, source: &str) -> Vec<FuncSpec> {
        let mut functions = Vec::new();

        for func_item in NodeHelper::find_all(root, "function_item") {
            // Skip functions inside impl blocks (those are methods)
            if self.is_inside_impl(func_item) {
                continue;
            }

            if let Some(spec) = self.extract_func_spec(func_item, source, None) {
                functions.push(spec);
            }
        }

        functions
    }

    fn is_inside_impl(&self, node: Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "impl_item" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn extract_methods(&self, root: Node, source: &str) -> Vec<FuncSpec> {
        let mut methods = Vec::new();

        for impl_item in NodeHelper::find_all(root, "impl_item") {
            let receiver = self.get_impl_type(impl_item, source);

            if let Some(body) = NodeHelper::child_by_kind(impl_item, "declaration_list") {
                for func in NodeHelper::find_all(body, "function_item") {
                    if let Some(spec) = self.extract_func_spec(func, source, receiver.clone()) {
                        methods.push(spec);
                    }
                }
            }
        }

        methods
    }

    fn get_impl_type(&self, impl_item: Node, source: &str) -> Option<String> {
        NodeHelper::field(impl_item, "type")
            .map(|n| NodeHelper::text(n, source).to_string())
    }

    fn extract_func_spec(&self, node: Node, source: &str, receiver: Option<String>) -> Option<FuncSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(node, "parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("()");

        let return_type = NodeHelper::child_by_kind(node, "return_type")
            .map(|n| format!(" -> {}", NodeHelper::text(n, source)))
            .unwrap_or_default();

        let type_params = NodeHelper::child_by_kind(node, "type_parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or_default();

        let signature = format!("fn {}{}{}{}", name, type_params, params, return_type);
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

        for const_item in NodeHelper::find_all(root, "const_item") {
            let name = NodeHelper::field(const_item, "name")
                .map(|n| NodeHelper::text(n, source).to_string());

            let type_name = NodeHelper::field(const_item, "type")
                .map(|n| NodeHelper::text(n, source).to_string());

            let value = NodeHelper::field(const_item, "value")
                .map(|n| NodeHelper::text(n, source).to_string());

            let doc = NodeHelper::preceding_comment(const_item, source);

            if let Some(name) = name {
                constants.push(ConstSpec {
                    name,
                    type_name,
                    value,
                    doc,
                });
            }
        }

        constants
    }

    fn extract_statics(&self, root: Node, source: &str) -> Vec<VarSpec> {
        let mut statics = Vec::new();

        for static_item in NodeHelper::find_all(root, "static_item") {
            let name = NodeHelper::field(static_item, "name")
                .map(|n| NodeHelper::text(n, source).to_string());

            let type_name = NodeHelper::field(static_item, "type")
                .map(|n| NodeHelper::text(n, source).to_string());

            let doc = NodeHelper::preceding_comment(static_item, source);

            if let Some(name) = name {
                statics.push(VarSpec {
                    name,
                    type_name,
                    doc,
                });
            }
        }

        statics
    }
}

impl Default for RustPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePlugin for RustPlugin {
    fn name(&self) -> &'static str {
        "rust"
    }

    fn extensions(&self) -> &[&'static str] {
        &["rs"]
    }

    fn extract(&self, source: &str, path: &Path) -> Result<FileSpec> {
        let language = tree_sitter_rust::LANGUAGE;
        let mut parser = TreeSitterParser::new(language.into())?;
        let tree = parser.parse(source)?;
        let root = tree.root_node();

        Ok(FileSpec {
            file: path.to_string_lossy().to_string(),
            package: self.extract_module(path),
            imports: self.extract_uses(root, source),
            types: self.extract_types(root, source),
            functions: self.extract_functions(root, source),
            methods: self.extract_methods(root, source),
            constants: self.extract_constants(root, source),
            variables: self.extract_statics(root, source),
            errors: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_struct() {
        let plugin = RustPlugin::new();
        let source = r#"
/// A user in the system.
pub struct User {
    id: u64,
    name: String,
}
"#;
        let spec = plugin.extract(source, Path::new("user.rs")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "User struct");
        assert!(spec.types[0].doc.as_ref().unwrap().contains("user in the system"));
    }

    #[test]
    fn test_extract_enum() {
        let plugin = RustPlugin::new();
        let source = r#"
pub enum Status {
    Active,
    Inactive,
    Pending,
}
"#;
        let spec = plugin.extract(source, Path::new("status.rs")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "Status enum");
        assert_eq!(spec.types[0].variants.len(), 3);
    }

    #[test]
    fn test_extract_trait() {
        let plugin = RustPlugin::new();
        let source = r#"
pub trait Drawable {
    fn draw(&self);
    fn bounds(&self) -> Rect;
}
"#;
        let spec = plugin.extract(source, Path::new("drawable.rs")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "Drawable trait");
        assert_eq!(spec.types[0].kind, "trait");
    }

    #[test]
    fn test_extract_impl_methods() {
        let plugin = RustPlugin::new();
        let source = r#"
struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}", self.name)
    }
}
"#;
        let spec = plugin.extract(source, Path::new("user.rs")).unwrap();
        assert_eq!(spec.methods.len(), 2);
        assert!(spec.methods[0].receiver.is_some());
    }
}
