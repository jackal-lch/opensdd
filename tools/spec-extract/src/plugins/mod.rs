mod go;
mod python;
mod rust;
mod typescript;

pub use go::GoPlugin;
pub use python::PythonPlugin;
pub use rust::RustPlugin;
pub use typescript::TypeScriptPlugin;

use crate::parser::LanguagePlugin;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Registry of all available language plugins.
pub struct PluginRegistry {
    plugins: HashMap<String, Arc<dyn LanguagePlugin>>,
    ext_map: HashMap<String, String>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    /// Create a new registry with all built-in plugins.
    pub fn new() -> Self {
        let mut registry = Self {
            plugins: HashMap::new(),
            ext_map: HashMap::new(),
        };

        registry.register(Arc::new(GoPlugin::new()));
        registry.register(Arc::new(RustPlugin::new()));
        registry.register(Arc::new(PythonPlugin::new()));
        registry.register(Arc::new(TypeScriptPlugin::new()));

        registry
    }

    /// Register a plugin.
    pub fn register(&mut self, plugin: Arc<dyn LanguagePlugin>) {
        let name = plugin.name().to_string();
        for ext in plugin.extensions() {
            self.ext_map.insert(ext.to_string(), name.clone());
        }
        self.plugins.insert(name, plugin);
    }

    /// Get a plugin by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn LanguagePlugin>> {
        self.plugins.get(name).cloned()
    }

    /// Get a plugin that can handle the given file path.
    pub fn get_for_file(&self, path: &Path) -> Option<Arc<dyn LanguagePlugin>> {
        let ext = path.extension()?.to_str()?;
        let name = self.ext_map.get(ext)?;
        self.plugins.get(name).cloned()
    }

    /// Get all registered plugin names.
    pub fn names(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a file can be handled by any plugin.
    pub fn can_handle(&self, path: &Path) -> bool {
        self.get_for_file(path).is_some()
    }

    /// Get all registered extensions.
    pub fn extensions(&self) -> Vec<&str> {
        self.ext_map.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_registry_has_all_plugins() {
        let registry = PluginRegistry::new();
        assert!(registry.get("go").is_some());
        assert!(registry.get("rust").is_some());
        assert!(registry.get("python").is_some());
        assert!(registry.get("typescript").is_some());
    }

    #[test]
    fn test_get_for_file() {
        let registry = PluginRegistry::new();

        let go_file = PathBuf::from("main.go");
        assert_eq!(registry.get_for_file(&go_file).unwrap().name(), "go");

        let rs_file = PathBuf::from("lib.rs");
        assert_eq!(registry.get_for_file(&rs_file).unwrap().name(), "rust");

        let py_file = PathBuf::from("app.py");
        assert_eq!(registry.get_for_file(&py_file).unwrap().name(), "python");

        let ts_file = PathBuf::from("index.ts");
        assert_eq!(registry.get_for_file(&ts_file).unwrap().name(), "typescript");

        let tsx_file = PathBuf::from("App.tsx");
        assert_eq!(registry.get_for_file(&tsx_file).unwrap().name(), "typescript");
    }

    #[test]
    fn test_unknown_extension() {
        let registry = PluginRegistry::new();
        let unknown = PathBuf::from("file.xyz");
        assert!(registry.get_for_file(&unknown).is_none());
    }
}
