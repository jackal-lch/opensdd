use std::fmt;

/// Maximum number of retries.
pub const MAX_RETRIES: u32 = 3;

/// A user in the system.
#[derive(Debug, Clone)]
pub struct User {
    id: u64,
    name: String,
}

/// User roles in the system.
pub enum Role {
    Admin,
    Editor,
    Viewer,
}

/// Trait for greeting behavior.
pub trait Greeter {
    fn greet(&self) -> String;
    fn farewell(&self) -> String;
}

impl User {
    /// Create a new user with the given name.
    pub fn new(name: String) -> Self {
        Self { id: 0, name }
    }

    /// Get the user's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the user's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Greeter for User {
    fn greet(&self) -> String {
        format!("Hello, {}", self.name)
    }

    fn farewell(&self) -> String {
        format!("Goodbye, {}", self.name)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User({})", self.name)
    }
}

/// Create a default user.
pub fn default_user() -> User {
    User::new("Anonymous".to_string())
}
