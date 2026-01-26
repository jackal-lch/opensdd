use anyhow::Result;
use std::path::Path;
use tree_sitter::Node;

use crate::parser::{LanguagePlugin, NodeHelper, TreeSitterParser};
use crate::spec::{ConstSpec, FileSpec, FuncSpec, TypeSpec, VarSpec};

/// Plugin for extracting specifications from TypeScript source files.
pub struct TypeScriptPlugin;

impl TypeScriptPlugin {
    pub fn new() -> Self {
        Self
    }

    fn get_language(&self, path: &Path) -> tree_sitter::Language {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext == "tsx" {
            tree_sitter_typescript::LANGUAGE_TSX.into()
        } else {
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
        }
    }

    fn extract_module(&self, path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    fn extract_imports(&self, root: Node, source: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for import_stmt in NodeHelper::find_all(root, "import_statement") {
            if let Some(source_node) = NodeHelper::field(import_stmt, "source") {
                let import_path = NodeHelper::text(source_node, source)
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();
                imports.push(import_path);
            }
        }

        imports
    }

    fn extract_types(&self, root: Node, source: &str) -> Vec<TypeSpec> {
        let mut types = Vec::new();

        // Extract interfaces
        for interface_decl in NodeHelper::find_all(root, "interface_declaration") {
            if let Some(spec) = self.extract_interface(interface_decl, source) {
                types.push(spec);
            }
        }

        // Extract classes
        for class_decl in NodeHelper::find_all(root, "class_declaration") {
            if let Some(spec) = self.extract_class(class_decl, source) {
                types.push(spec);
            }
        }

        // Extract type aliases
        for type_alias in NodeHelper::find_all(root, "type_alias_declaration") {
            if let Some(spec) = self.extract_type_alias(type_alias, source) {
                types.push(spec);
            }
        }

        // Extract enums
        for enum_decl in NodeHelper::find_all(root, "enum_declaration") {
            if let Some(spec) = self.extract_enum(enum_decl, source) {
                types.push(spec);
            }
        }

        types
    }

    fn extract_interface(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut methods = Vec::new();
        let mut fields = Vec::new();
        let mut type_params = Vec::new();
        let mut embeds = Vec::new();

        // Extract type parameters
        if let Some(params) = NodeHelper::child_by_kind(node, "type_parameters") {
            let mut cursor = params.walk();
            for child in params.children(&mut cursor) {
                if child.kind() == "type_parameter" {
                    type_params.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract extends clause
        if let Some(heritage) = NodeHelper::child_by_kind(node, "extends_clause") {
            let mut cursor = heritage.walk();
            for child in heritage.children(&mut cursor) {
                if child.kind() == "type_identifier" || child.kind() == "generic_type" {
                    embeds.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract members - check various possible body node names
        let body = NodeHelper::child_by_kind(node, "object_type")
            .or_else(|| NodeHelper::child_by_kind(node, "interface_body"));
        if let Some(body) = body {
            self.extract_interface_members(body, source, &mut fields, &mut methods);
        }

        Some(TypeSpec {
            name: format!("{} interface", name),
            doc,
            kind: "interface".to_string(),
            fields,
            methods,
            type_params,
            embeds,
            ..Default::default()
        })
    }

    fn extract_interface_members(
        &self,
        body: Node,
        source: &str,
        fields: &mut Vec<String>,
        methods: &mut Vec<String>,
    ) {
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            match child.kind() {
                "property_signature" => {
                    let name = NodeHelper::field(child, "name")
                        .map(|n| NodeHelper::text(n, source));
                    let type_ann = NodeHelper::child_by_kind(child, "type_annotation")
                        .map(|n| NodeHelper::text(n, source));

                    if let Some(name) = name {
                        let field = if let Some(t) = type_ann {
                            format!("{}{}", name, t)
                        } else {
                            name.to_string()
                        };
                        fields.push(field);
                    }
                }
                "method_signature" => {
                    let name = NodeHelper::field(child, "name")
                        .map(|n| NodeHelper::text(n, source));
                    let params = NodeHelper::field(child, "parameters")
                        .map(|n| NodeHelper::text(n, source))
                        .unwrap_or("()");
                    let return_type = NodeHelper::child_by_kind(child, "type_annotation")
                        .map(|n| NodeHelper::text(n, source))
                        .unwrap_or("");

                    if let Some(name) = name {
                        methods.push(format!("{}{}{}", name, params, return_type));
                    }
                }
                _ => {}
            }
        }
    }

    fn extract_class(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut methods = Vec::new();
        let mut fields = Vec::new();
        let mut type_params = Vec::new();
        let mut embeds = Vec::new();
        let mut implements = None;

        // Extract type parameters
        if let Some(params) = NodeHelper::child_by_kind(node, "type_parameters") {
            let mut cursor = params.walk();
            for child in params.children(&mut cursor) {
                if child.kind() == "type_parameter" {
                    type_params.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        // Extract extends clause
        if let Some(heritage) = NodeHelper::child_by_kind(node, "class_heritage") {
            let mut cursor = heritage.walk();
            for child in heritage.children(&mut cursor) {
                if child.kind() == "extends_clause" {
                    if let Some(type_node) = child.child(1) {
                        embeds.push(NodeHelper::text(type_node, source).to_string());
                    }
                }
                if child.kind() == "implements_clause" {
                    let mut impl_cursor = child.walk();
                    let impl_types: Vec<String> = child
                        .children(&mut impl_cursor)
                        .filter(|c| c.kind() == "type_identifier" || c.kind() == "generic_type")
                        .map(|c| NodeHelper::text(c, source).to_string())
                        .collect();
                    if !impl_types.is_empty() {
                        implements = Some(impl_types.join(", "));
                    }
                }
            }
        }

        // Extract members
        if let Some(body) = NodeHelper::child_by_kind(node, "class_body") {
            self.extract_class_members(body, source, &mut fields, &mut methods);
        }

        Some(TypeSpec {
            name: format!("{} class", name),
            doc,
            kind: "class".to_string(),
            fields,
            methods,
            type_params,
            embeds,
            implements,
            ..Default::default()
        })
    }

    fn extract_class_members(
        &self,
        body: Node,
        source: &str,
        fields: &mut Vec<String>,
        methods: &mut Vec<String>,
    ) {
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            match child.kind() {
                "public_field_definition" | "property_declaration" => {
                    let name = NodeHelper::field(child, "name")
                        .map(|n| NodeHelper::text(n, source));
                    let type_ann = NodeHelper::child_by_kind(child, "type_annotation")
                        .map(|n| NodeHelper::text(n, source));

                    if let Some(name) = name {
                        let field = if let Some(t) = type_ann {
                            format!("{}{}", name, t)
                        } else {
                            name.to_string()
                        };
                        fields.push(field);
                    }
                }
                "method_definition" => {
                    let name = NodeHelper::field(child, "name")
                        .map(|n| NodeHelper::text(n, source));
                    let params = NodeHelper::field(child, "parameters")
                        .map(|n| NodeHelper::text(n, source))
                        .unwrap_or("()");
                    let return_type = NodeHelper::child_by_kind(child, "type_annotation")
                        .map(|n| NodeHelper::text(n, source))
                        .unwrap_or("");

                    if let Some(name) = name {
                        methods.push(format!("{}{}{}", name, params, return_type));
                    }
                }
                _ => {}
            }
        }
    }

    fn extract_type_alias(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut type_params = Vec::new();

        // Extract type parameters
        if let Some(params) = NodeHelper::child_by_kind(node, "type_parameters") {
            let mut cursor = params.walk();
            for child in params.children(&mut cursor) {
                if child.kind() == "type_parameter" {
                    type_params.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        Some(TypeSpec {
            name: format!("{} type", name),
            doc,
            kind: "type_alias".to_string(),
            type_params,
            ..Default::default()
        })
    }

    fn extract_enum(&self, node: Node, source: &str) -> Option<TypeSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source).to_string())?;

        let doc = NodeHelper::preceding_comment(node, source);

        let mut variants = Vec::new();

        if let Some(body) = NodeHelper::child_by_kind(node, "enum_body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                if child.kind() == "property_identifier" || child.kind() == "enum_assignment" {
                    variants.push(NodeHelper::text(child, source).to_string());
                }
            }
        }

        Some(TypeSpec {
            name: format!("{} enum", name),
            doc,
            kind: "enum".to_string(),
            variants,
            ..Default::default()
        })
    }

    fn extract_functions(&self, root: Node, source: &str) -> Vec<FuncSpec> {
        let mut functions = Vec::new();

        // Regular function declarations
        for func_decl in NodeHelper::find_all(root, "function_declaration") {
            if self.is_inside_class(func_decl) {
                continue;
            }
            if let Some(spec) = self.extract_func_spec(func_decl, source) {
                functions.push(spec);
            }
        }

        // Arrow functions assigned to const/let
        for var_decl in NodeHelper::find_all(root, "lexical_declaration") {
            if self.is_inside_class(var_decl) {
                continue;
            }
            for declarator in NodeHelper::children_by_kind(var_decl, "variable_declarator") {
                let value = NodeHelper::field(declarator, "value");
                if let Some(v) = value {
                    if v.kind() == "arrow_function" {
                        if let Some(spec) = self.extract_arrow_func(declarator, v, source) {
                            functions.push(spec);
                        }
                    }
                }
            }
        }

        functions
    }

    fn is_inside_class(&self, node: Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "class_declaration" || parent.kind() == "class" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn extract_func_spec(&self, node: Node, source: &str) -> Option<FuncSpec> {
        let name = NodeHelper::field(node, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(node, "parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("()");

        let return_type = NodeHelper::child_by_kind(node, "type_annotation")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or("");

        let type_params = NodeHelper::child_by_kind(node, "type_parameters")
            .map(|n| NodeHelper::text(n, source))
            .unwrap_or_default();

        let signature = format!("function {}{}{}{}", name, type_params, params, return_type);
        let doc = NodeHelper::preceding_comment(node, source);

        Some(FuncSpec {
            signature,
            doc,
            ..Default::default()
        })
    }

    fn extract_arrow_func(&self, declarator: Node, arrow: Node, source: &str) -> Option<FuncSpec> {
        let name = NodeHelper::field(declarator, "name")
            .map(|n| NodeHelper::text(n, source))?;

        let params = NodeHelper::field(arrow, "parameters")
            .or_else(|| NodeHelper::field(arrow, "parameter"))
            .map(|n| {
                let text = NodeHelper::text(n, source);
                if text.starts_with('(') {
                    text.to_string()
                } else {
                    format!("({})", text)
                }
            })
            .unwrap_or_else(|| "()".to_string());

        let return_type = NodeHelper::child_by_kind(arrow, "type_annotation")
            .map(|n| NodeHelper::text(n, source).to_string())
            .unwrap_or_default();

        let signature = format!("const {} = {}{}", name, params, return_type);
        let doc = NodeHelper::preceding_comment(declarator.parent().unwrap_or(declarator), source);

        Some(FuncSpec {
            signature,
            doc,
            ..Default::default()
        })
    }

    fn extract_constants(&self, root: Node, source: &str) -> Vec<ConstSpec> {
        let mut constants = Vec::new();

        for var_decl in NodeHelper::find_all(root, "lexical_declaration") {
            if self.is_inside_class(var_decl) {
                continue;
            }

            // Check if it's a const declaration
            let kind = var_decl.child(0).map(|n| NodeHelper::text(n, source));
            if kind != Some("const") {
                continue;
            }

            for declarator in NodeHelper::children_by_kind(var_decl, "variable_declarator") {
                let name = NodeHelper::field(declarator, "name")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let value = NodeHelper::field(declarator, "value");

                // Skip if it's an arrow function
                if value.map(|v| v.kind() == "arrow_function").unwrap_or(false) {
                    continue;
                }

                let type_name = NodeHelper::child_by_kind(declarator, "type_annotation")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let value_str = value.map(|v| NodeHelper::text(v, source).to_string());

                let doc = NodeHelper::preceding_comment(var_decl, source);

                if let Some(name) = name {
                    constants.push(ConstSpec {
                        name,
                        type_name,
                        value: value_str,
                        doc,
                    });
                }
            }
        }

        constants
    }

    fn extract_variables(&self, root: Node, source: &str) -> Vec<VarSpec> {
        let mut variables = Vec::new();

        for var_decl in NodeHelper::find_all(root, "lexical_declaration") {
            if self.is_inside_class(var_decl) {
                continue;
            }

            // Check if it's a let declaration
            let kind = var_decl.child(0).map(|n| NodeHelper::text(n, source));
            if kind != Some("let") {
                continue;
            }

            for declarator in NodeHelper::children_by_kind(var_decl, "variable_declarator") {
                let name = NodeHelper::field(declarator, "name")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let type_name = NodeHelper::child_by_kind(declarator, "type_annotation")
                    .map(|n| NodeHelper::text(n, source).to_string());

                let doc = NodeHelper::preceding_comment(var_decl, source);

                if let Some(name) = name {
                    variables.push(VarSpec {
                        name,
                        type_name,
                        doc,
                    });
                }
            }
        }

        variables
    }
}

impl Default for TypeScriptPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePlugin for TypeScriptPlugin {
    fn name(&self) -> &'static str {
        "typescript"
    }

    fn extensions(&self) -> &[&'static str] {
        &["ts", "tsx"]
    }

    fn extract(&self, source: &str, path: &Path) -> Result<FileSpec> {
        let language = self.get_language(path);
        let mut parser = TreeSitterParser::new(language)?;
        let tree = parser.parse(source)?;
        let root = tree.root_node();

        Ok(FileSpec {
            file: path.to_string_lossy().to_string(),
            package: self.extract_module(path),
            imports: self.extract_imports(root, source),
            types: self.extract_types(root, source),
            functions: self.extract_functions(root, source),
            methods: Vec::new(), // TS methods are included in class types
            constants: self.extract_constants(root, source),
            variables: self.extract_variables(root, source),
            errors: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_interface() {
        let plugin = TypeScriptPlugin::new();
        let source = r#"
// A user in the system.
interface User {
    id: number;
    name: string;
    greet(): string;
}
"#;
        let spec = plugin.extract(source, Path::new("user.ts")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "User interface");
        assert_eq!(spec.types[0].kind, "interface");
    }

    #[test]
    fn test_extract_class() {
        let plugin = TypeScriptPlugin::new();
        let source = r#"
class User implements Serializable {
    id: number;
    name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }

    greet(): string {
        return `Hello, ${this.name}`;
    }
}
"#;
        let spec = plugin.extract(source, Path::new("user.ts")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "User class");
        assert!(spec.types[0].implements.is_some());
    }

    #[test]
    fn test_extract_function() {
        let plugin = TypeScriptPlugin::new();
        let source = r#"
function hello(name: string): string {
    return `Hello, ${name}`;
}
"#;
        let spec = plugin.extract(source, Path::new("hello.ts")).unwrap();
        assert_eq!(spec.functions.len(), 1);
        assert!(spec.functions[0].signature.contains("hello"));
    }

    #[test]
    fn test_extract_arrow_function() {
        let plugin = TypeScriptPlugin::new();
        let source = r#"
const add = (a: number, b: number): number => a + b;
"#;
        let spec = plugin.extract(source, Path::new("math.ts")).unwrap();
        assert_eq!(spec.functions.len(), 1);
        assert!(spec.functions[0].signature.contains("add"));
    }

    #[test]
    fn test_extract_type_alias() {
        let plugin = TypeScriptPlugin::new();
        let source = r#"
type UserId = string;
type UserMap<T> = Map<UserId, T>;
"#;
        let spec = plugin.extract(source, Path::new("types.ts")).unwrap();
        assert_eq!(spec.types.len(), 2);
        assert_eq!(spec.types[0].name, "UserId type");
        assert_eq!(spec.types[1].name, "UserMap type");
    }

    #[test]
    fn test_extract_enum() {
        let plugin = TypeScriptPlugin::new();
        let source = r#"
enum Status {
    Active,
    Inactive,
    Pending
}
"#;
        let spec = plugin.extract(source, Path::new("status.ts")).unwrap();
        assert_eq!(spec.types.len(), 1);
        assert_eq!(spec.types[0].name, "Status enum");
    }
}
