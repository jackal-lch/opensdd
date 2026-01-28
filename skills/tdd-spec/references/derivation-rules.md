# Test Derivation Rules

Rules for transforming spec.yaml definitions into test cases.

## Overview

Tests are derived from three sources in spec.yaml:
1. **`for:` descriptions** → Behavioral expectations (happy path)
2. **`output:` types** → Success and error cases
3. **`input:` types** → Edge cases and boundaries

## Rule 1: Happy Path from `for:` Descriptions

The `for:` field describes what a function should do. Parse it to extract testable behaviors.

### Pattern: Action Verbs

| Verb in `for:` | Test Case Pattern |
|----------------|-------------------|
| "Authenticate X" | "authenticates X successfully" |
| "Create X" | "creates X" |
| "Validate X" | "validates X" |
| "Process X" | "processes X correctly" |
| "Return X" | "returns X" |
| "Transform X to Y" | "transforms X to Y" |

### Pattern: Compound Descriptions

When `for:` contains "and", create multiple tests:

```yaml
# Spec
for: "Authenticate user credentials and create session"

# Derived tests
- "authenticates valid user credentials"
- "creates session on successful authentication"
```

### Pattern: Implicit Behaviors

Some behaviors are implied by the action:

| Action | Implied Test |
|--------|--------------|
| "Create X" | "returns created X with ID" |
| "Update X" | "returns updated X" |
| "Delete X" | "returns success confirmation" |
| "Find X" | "returns X when found" |
| "Validate X" | "returns valid when X is valid" |

## Rule 2: Error Cases from Output Union Types

When `output:` is a union type with error types, each error type gets a test.

### Pattern: Union Types

```yaml
# Spec
output: AuthResult | AuthError

# Derived tests
- "returns AuthResult for valid credentials" (happy path)
- "returns AuthError for invalid credentials"
```

### Pattern: Multiple Error Types

```yaml
# Spec
output: User | UserNotFound | ValidationError | DatabaseError

# Derived tests
- "returns User when found" (happy path)
- "returns UserNotFound when user doesn't exist"
- "returns ValidationError when input is invalid"
- "returns DatabaseError when database fails"
```

### Pattern: Error Type Analysis

For each error type, infer the condition that causes it:

| Error Type Pattern | Inferred Condition |
|-------------------|-------------------|
| `NotFound` | "when X doesn't exist" |
| `ValidationError` | "when input is invalid" |
| `AuthError` | "when authentication fails" |
| `PermissionError` | "when user lacks permission" |
| `ConflictError` | "when resource already exists" |
| `TimeoutError` | "when operation times out" |

## Rule 3: Edge Cases from Input Types

Analyze input types to generate edge case tests.

### Universal Edge Cases

Always include these for any input:

| Edge Case | Test Name Pattern |
|-----------|-------------------|
| null/undefined | "handles null input" |
| empty string | "handles empty string" |
| empty array | "handles empty array" |
| empty object | "handles empty object" |

### Type-Specific Edge Cases

| Input Type | Edge Cases |
|------------|------------|
| `string` | empty, whitespace, special chars, max length |
| `number` | 0, negative, MAX_INT, MIN_INT, NaN |
| `array` | empty, single item, max items |
| `date` | past, future, epoch, invalid |
| `email` | invalid format, special chars |
| `url` | invalid format, relative, absolute |
| `id` | invalid format, non-existent |

### Pattern: Required vs Optional Fields

```yaml
# Spec (implied from type)
input:
  type: CreateUserInput
  fields:
    email: string (required)
    name: string (optional)

# Derived tests
- "handles missing optional name"
- "rejects missing required email"
```

## Rule 4: Event Tests from `emits:`

Each event in `emits:` gets an emission test.

### Pattern: Event Emission

```yaml
# Spec
emits:
  - UserCreated:
      for: "Notify when user is created"
      payload: UserId

# Derived test
- "emits UserCreated event with UserId on successful creation"
```

### Pattern: Event Timing

The test should verify:
1. Event is emitted
2. Event contains correct payload
3. Event is emitted at correct time (after success, not on failure)

```yaml
# Derived tests
- "emits UserCreated on successful creation"
- "does not emit UserCreated on creation failure"
```

## Rule 5: Handler Tests from `subscribes:`

Each subscription gets a handler test.

### Pattern: Event Handling

```yaml
# Spec
subscribes:
  - UserDeleted: "Clean up user data"

# Derived tests
- "handles UserDeleted event"
- "cleans up user data when UserDeleted received"
```

## Rule 6: Test Type Classification

Assign test type based on component layer and function characteristics.

### Layer-Based Classification

| Component Layer | Default Test Type |
|-----------------|-------------------|
| domain | unit |
| application | integration |
| infrastructure | integration |

### Override Rules

Force unit test when:
- Function is pure (no side effects)
- Function only uses other domain components

Force integration test when:
- Function interacts with external services
- Function reads/writes to database
- Function makes HTTP calls
- Function uses file system

## Derivation Checklist

For each function in `provides:`:

- [ ] At least 1 happy path test from `for:`
- [ ] 1 error test per error type in `output:`
- [ ] Edge case tests for required inputs
- [ ] Null/empty handling test

For each event in `emits:`:

- [ ] Event emission test on success path
- [ ] Event non-emission test on failure path (optional)

For each subscription in `subscribes:`:

- [ ] Event handler test

## Example: Complete Derivation

```yaml
# spec.yaml
components:
  AuthService:
    for: "Handle user authentication and session management"
    layer: application
    provides:
      - login:
          for: "Authenticate user credentials and create session"
          input: Credentials
          output: AuthResult | AuthError
      - logout:
          for: "Terminate user session"
          input: SessionId
          output: void | SessionError
    emits:
      - UserLoggedIn:
          for: "Notify when user successfully logs in"
          payload: UserId
    subscribes:
      - UserDeleted: "Terminate all sessions for deleted user"
```

### Derived Test Cases

```yaml
test_cases:
  - function: login
    cases:
      # Happy path (from for:)
      - name: "authenticates valid credentials"
        category: happy
        test_type: integration
      - name: "creates session on successful login"
        category: happy
        test_type: integration
      # Error cases (from output union)
      - name: "returns AuthError for invalid password"
        category: error
        test_type: unit
      - name: "returns AuthError for unknown user"
        category: error
        test_type: unit
      # Edge cases (from input type)
      - name: "handles null credentials"
        category: edge
        test_type: unit
      - name: "handles empty email"
        category: edge
        test_type: unit
      - name: "handles empty password"
        category: edge
        test_type: unit

  - function: logout
    cases:
      - name: "terminates existing session"
        category: happy
        test_type: integration
      - name: "returns SessionError for invalid session"
        category: error
        test_type: unit
      - name: "handles null session ID"
        category: edge
        test_type: unit

  - function: events
    cases:
      - name: "emits UserLoggedIn on successful login"
        category: event
        test_type: integration
      - name: "does not emit UserLoggedIn on failed login"
        category: event
        test_type: unit

  - function: subscriptions
    cases:
      - name: "handles UserDeleted event"
        category: event
        test_type: integration
      - name: "terminates all sessions when UserDeleted received"
        category: event
        test_type: integration
```
