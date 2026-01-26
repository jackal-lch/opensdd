# Rust

Options and trade-offs for Rust projects. AI decides based on blueprint context.

## Conventions (fixed)

| Category | Convention |
|----------|------------|
| types | PascalCase |
| functions | snake_case |
| modules | snake_case |
| constants | UPPER_SNAKE_CASE |
| files | snake_case |
| crates | kebab-case (Cargo.toml) or snake_case |

## Structure Options

| Option | Description | When to Consider |
|--------|-------------|------------------|
| flat | Single src/ with modules | Simple binaries, small projects |
| layered | domain/, services/, handlers/ | Web services |
| workspace | Multiple crates | Large projects, shared code, independent versioning |
| lib + bin | Library with binary wrapper | Reusable logic with CLI |

**Rust-specific:**
- `mod.rs` or `module_name.rs` for module roots (2018+ allows both)
- Workspace adds compile overhead, don't use for <5 crates
- `pub(crate)` for internal visibility

## Pattern Options

| Pattern | Rust Idiom |
|---------|------------|
| Traits | Define behavior, implement for types |
| Newtype | `struct {Id}(inner)` wrapper for type safety |
| Builder | For complex construction with validation |
| Error handling | `thiserror` (libraries), `anyhow` (applications) |
| Async | tokio runtime, async/await |

## Trade-offs

| Choice | Pros | Cons |
|--------|------|------|
| Workspace | Parallel compilation, clear boundaries | Setup overhead, dependency management |
| Single crate | Simple | Longer compile times as it grows |
| thiserror | Typed errors, pattern matching | More boilerplate |
| anyhow | Convenient, context chaining | No typed matching |
| sqlx | Compile-time checked SQL | Slower compilation |
| diesel | Full ORM, migrations | Learning curve, macros |

## Preferred / Avoid

| Category | Prefer | Avoid |
|----------|--------|-------|
| HTTP framework | axum, actix-web | - |
| Async runtime | tokio | async-std (less ecosystem) |
| Serialization | serde | - |
| Error (library) | thiserror | - |
| Error (app) | anyhow | - |
| Logging | tracing | log (less features) |

## Testing

- `#[cfg(test)]` module in same file (unit)
- `tests/` directory (integration)
- `#[tokio::test]` for async
- No mocking framework needed (use traits)
