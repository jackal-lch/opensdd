use anyhow::{Context, Result};
use tree_sitter::{Language, Node, Parser, Tree};

/// Wrapper around tree-sitter Parser with utility methods.
pub struct TreeSitterParser {
    parser: Parser,
}

impl TreeSitterParser {
    /// Create a new parser for the given language.
    pub fn new(language: Language) -> Result<Self> {
        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .context("Failed to set parser language")?;
        Ok(Self { parser })
    }

    /// Parse source code into a syntax tree.
    pub fn parse(&mut self, source: &str) -> Result<Tree> {
        self.parser
            .parse(source, None)
            .context("Failed to parse source code")
    }
}

/// Helper functions for working with tree-sitter nodes.
pub struct NodeHelper;

impl NodeHelper {
    /// Get the text content of a node from the source.
    pub fn text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
        node.utf8_text(source.as_bytes()).unwrap_or("")
    }

    /// Find the first child with the given kind.
    pub fn child_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
        let mut cursor = node.walk();
        let result = node.children(&mut cursor)
            .find(|child| child.kind() == kind);
        result
    }

    /// Find all children with the given kind.
    pub fn children_by_kind<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .filter(|child| child.kind() == kind)
            .collect()
    }

    /// Get a named child by field name.
    pub fn field<'a>(node: Node<'a>, name: &str) -> Option<Node<'a>> {
        node.child_by_field_name(name)
    }

    /// Get the text of a named field.
    pub fn field_text<'a>(node: Node<'a>, name: &str, source: &'a str) -> Option<&'a str> {
        node.child_by_field_name(name)
            .map(|n| Self::text(n, source))
    }

    /// Check if a node has a child with the given kind.
    pub fn has_child_kind(node: Node, kind: &str) -> bool {
        let mut cursor = node.walk();
        let result = node.children(&mut cursor).any(|child| child.kind() == kind);
        result
    }

    /// Find the previous sibling that is a comment.
    pub fn preceding_comment<'a>(node: Node<'a>, source: &'a str) -> Option<String> {
        let mut current = node;
        let mut comments = Vec::new();

        // Walk backwards through siblings to collect comments
        while let Some(prev) = current.prev_sibling() {
            let kind = prev.kind();
            if kind == "comment" || kind == "line_comment" || kind == "block_comment" {
                comments.push(Self::clean_comment(Self::text(prev, source)));
                current = prev;
            } else if kind.contains("whitespace") || prev.is_extra() {
                current = prev;
            } else {
                break;
            }
        }

        if comments.is_empty() {
            None
        } else {
            comments.reverse();
            Some(comments.join("\n"))
        }
    }

    /// Clean a comment by removing comment markers.
    pub fn clean_comment(comment: &str) -> String {
        let lines: Vec<&str> = comment.lines().collect();
        let cleaned: Vec<String> = lines
            .iter()
            .map(|line| {
                let trimmed = line.trim();
                // Handle various comment styles
                if trimmed.starts_with("///") {
                    trimmed[3..].trim().to_string()
                } else if trimmed.starts_with("//!") {
                    trimmed[3..].trim().to_string()
                } else if trimmed.starts_with("//") {
                    trimmed[2..].trim().to_string()
                } else if trimmed.starts_with("/*") {
                    trimmed[2..].trim_start().trim_end_matches("*/").trim().to_string()
                } else if trimmed.starts_with("*/") {
                    String::new()
                } else if trimmed.starts_with('*') {
                    trimmed[1..].trim().to_string()
                } else if trimmed.starts_with('#') {
                    trimmed[1..].trim().to_string()
                } else if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
                    trimmed[3..].trim_end_matches("\"\"\"").trim_end_matches("'''").trim().to_string()
                } else {
                    trimmed.to_string()
                }
            })
            .filter(|s| !s.is_empty())
            .collect();

        cleaned.join("\n")
    }

    /// Recursively find all nodes of a given kind.
    pub fn find_all<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
        let mut results = Vec::new();
        Self::find_all_recursive(node, kind, &mut results);
        results
    }

    fn find_all_recursive<'a>(node: Node<'a>, kind: &str, results: &mut Vec<Node<'a>>) {
        if node.kind() == kind {
            results.push(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::find_all_recursive(child, kind, results);
        }
    }

    /// Get the line number (1-indexed) of a node.
    pub fn line_number(node: Node) -> usize {
        node.start_position().row + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_comment_line() {
        assert_eq!(NodeHelper::clean_comment("// Hello world"), "Hello world");
        assert_eq!(NodeHelper::clean_comment("/// Doc comment"), "Doc comment");
    }

    #[test]
    fn test_clean_comment_python() {
        assert_eq!(NodeHelper::clean_comment("# Python comment"), "Python comment");
    }
}
