# Python

Options and trade-offs for Python projects. AI decides based on blueprint context.

## Conventions (fixed)

| Category | Convention |
|----------|------------|
| classes | PascalCase |
| functions | snake_case |
| modules | snake_case |
| constants | UPPER_SNAKE_CASE |
| files | snake_case |
| private | `_prefix` (convention), `__prefix` (name mangling) |

## Structure Options

| Option | Description | When to Consider |
|--------|-------------|------------------|
| flat | Single package | Scripts, simple tools |
| src/ layout | `src/package_name/` | Libraries, prevents import confusion |
| by feature | Feature folders with `__init__.py` | Web apps, domain-driven |
| monorepo | Multiple packages, shared deps | Large orgs, multiple services |

**Python-specific:**
- `__init__.py` required for packages (or namespace packages)
- `pyproject.toml` is the modern standard
- Virtual environments are essential

## Pattern Options

| Pattern | Python Idiom |
|---------|--------------|
| Interfaces | Protocol (typing), ABC (runtime) |
| Dependency injection | Constructor, or dependency-injector library |
| Error handling | Exceptions with custom hierarchy |
| Data classes | dataclass, pydantic, attrs |
| Async | asyncio, async/await |

## Trade-offs

| Choice | Pros | Cons |
|--------|------|------|
| pydantic | Validation, serialization, settings | Runtime overhead |
| dataclass | Stdlib, simple | No validation |
| attrs | Flexible, performant | External dep |
| SQLAlchemy | Full ORM, mature | Complex, implicit |
| Raw SQL | Explicit, simple | No abstraction |
| FastAPI | Async, OpenAPI, pydantic | Newer |
| Flask | Simple, flexible | Sync by default |
| Django | Batteries included | Monolithic |

## Preferred / Avoid

| Category | Prefer | Avoid |
|----------|--------|-------|
| Project config | pyproject.toml | setup.py (legacy) |
| Dependency management | uv, poetry | pip + requirements.txt (for apps) |
| Type checking | mypy, pyright | No types |
| Data validation | pydantic | Manual validation |
| Testing | pytest | unittest (verbose) |

## Testing

- pytest (de facto standard)
- `tests/` directory (separate)
- Fixtures for setup/teardown
- `pytest-asyncio` for async
