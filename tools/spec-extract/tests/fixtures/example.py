"""Example Python module for spec extraction testing."""

from typing import Optional, List
from dataclasses import dataclass

MAX_RETRIES = 3
DEFAULT_NAME = "Anonymous"


@dataclass
class User:
    """Represents a user in the system."""

    id: int
    name: str
    email: Optional[str] = None

    def greet(self) -> str:
        """Return a greeting for the user."""
        return f"Hello, {self.name}"

    def set_name(self, name: str) -> None:
        """Update the user's name."""
        self.name = name


class Admin(User):
    """An admin user with elevated privileges."""

    permissions: List[str]

    def __init__(self, id: int, name: str, permissions: List[str] = None):
        super().__init__(id, name)
        self.permissions = permissions or []

    def has_permission(self, perm: str) -> bool:
        """Check if admin has a specific permission."""
        return perm in self.permissions


def create_user(name: str, email: Optional[str] = None) -> User:
    """Create a new user with the given name and optional email."""
    return User(id=0, name=name, email=email)


def validate_email(email: str) -> bool:
    """Validate an email address format."""
    return "@" in email and "." in email
