# TypeScript

Options and trade-offs for TypeScript projects. AI decides based on blueprint context.

## Conventions (fixed)

| Category | Convention |
|----------|------------|
| types/interfaces | PascalCase |
| functions | camelCase |
| constants | UPPER_SNAKE_CASE or camelCase |
| files | camelCase or kebab-case |
| private | `private` keyword, `#` prefix (ES2022) |

## Structure Options

| Option | Description | When to Consider |
|--------|-------------|------------------|
| flat | Single src/ | Small projects, scripts |
| by layer | domain/, application/, infrastructure/ | Clean architecture |
| by feature | Feature folders with index.ts | Most web apps |
| monorepo | Turborepo, Nx, pnpm workspaces | Multiple packages, shared code |

**TypeScript-specific:**
- `index.ts` for barrel exports
- Path aliases (`@/`) in tsconfig
- Strict mode recommended

## Pattern Options

| Pattern | TypeScript Idiom |
|---------|------------------|
| Interfaces | `interface` for objects, `type` for unions |
| Dependency injection | Constructor, or InversifyJS, tsyringe |
| Error handling | Exceptions, or Result<T, E> pattern |
| Data validation | Zod, io-ts (runtime), types (compile-time only) |
| Async | async/await, Promises |

## Trade-offs

| Choice | Pros | Cons |
|--------|------|------|
| NestJS | Structured, decorators, DI | Opinionated, learning curve |
| Express | Simple, flexible | Minimal structure |
| Fastify | Performance, schema validation | Less ecosystem |
| Zod | Runtime validation, type inference | Runtime overhead |
| io-ts | Functional, composable | Learning curve |
| Prisma | Type-safe, migrations | Build step, lock-in |
| Drizzle | SQL-like, lightweight | Newer |
| TypeORM | Traditional ORM | Complex, decorators |

## Preferred / Avoid

| Category | Prefer | Avoid |
|----------|--------|-------|
| Runtime | Node.js, Bun, Deno | - |
| Package manager | pnpm | npm (slower, larger) |
| Testing | vitest, jest | mocha (more setup) |
| Validation | zod | yup (less type inference) |
| Linting | eslint + typescript-eslint | tslint (deprecated) |

## Testing

- vitest or jest
- Tests alongside code (`*.spec.ts`) or in `__tests__/`
- Supertest for HTTP testing
- Mock with vi.mock or jest.mock
