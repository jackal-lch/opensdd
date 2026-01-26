# Go

Options and trade-offs for Go projects. AI decides based on blueprint context.

## Conventions (fixed)

| Category | Convention |
|----------|------------|
| types | PascalCase |
| functions | PascalCase (exported), camelCase (unexported) |
| files | lowercase, no underscores |
| packages | lowercase, singular (`user` not `users`) |
| constants | PascalCase (exported), camelCase (unexported) |
| errors | lowercase, no punctuation |

## Structure Options

| Option | Description | When to Consider |
|--------|-------------|------------------|
| flat | Single package | Simple tools, scripts, small CLIs |
| layered | handler → service → repository | Most web services |
| hexagonal | Ports & adapters, domain isolation | Complex domain, high testability needs |
| cmd/ + internal/ | Multiple binaries, private packages | Standard Go project layout |
| pkg/ | Public library code | Shared libraries, SDKs |

**Go-specific:**
- `internal/` is compiler-enforced private (unique to Go)
- One package = one directory (no sub-packages in same dir)

## Pattern Options

| Pattern | Go Idiom |
|---------|----------|
| Interfaces | Defined by consumer, not implementer. Small (1-3 methods) |
| Dependency injection | Constructor injection, no frameworks needed |
| Error handling | Wrap with `fmt.Errorf("context: %w", err)` |
| Concurrency | errgroup, worker pools, channels |
| Configuration | Functional options pattern, or config struct |

## Trade-offs

| Choice | Pros | Cons |
|--------|------|------|
| Small interfaces | Flexible, testable | More types |
| Large interfaces | Fewer types | Tight coupling |
| internal/ | Compiler-enforced privacy | Can't share across modules |
| pkg/ | Shareable | Public API commitment |
| Explicit SQL (sqlx) | Clear, debuggable | More code |
| ORM (gorm) | Less code | Magic, implicit behavior |

## Preferred / Avoid

| Category | Prefer | Avoid |
|----------|--------|-------|
| SQL | sqlx | gorm (magic) |
| HTTP router | chi, stdlib (1.22+) | gin (unless perf critical) |
| Logging | slog (stdlib) | logrus (deprecated pattern) |
| HTTP client | Custom with timeout | http.DefaultClient (no timeout) |

## Testing

- Table-driven tests (Go idiom)
- Tests in same package (`_test.go`)
- Integration tests with build tags (`//go:build integration`)
- Interfaces for mocking (no framework needed)
