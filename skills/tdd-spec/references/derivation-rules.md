# Test Derivation Rules

Algorithm for transforming spec.yaml into test cases.

**Core Principle:** Tests must verify behavior, not just types.

→ See: `rules.md` for TDD execution rules, anti-fake patterns, and test quality guidelines.

---

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

Assign test type based on component layer.

→ See: `lookup.md § Layer Classification`

---

## Test Structure Patterns

How to organize tests for clarity and comprehensive coverage.

### Pattern 1: Table-Driven Tests (Preferred for Multiple Cases)

When a function has multiple input/output combinations, use table-driven structure:

```python
# Python with pytest.parametrize
@pytest.mark.parametrize("input_data,expected_type,description", [
    (valid_credentials(), TokenPair, "valid credentials"),
    (wrong_password(), AuthError, "invalid password"),
    (unknown_email(), AuthError, "unknown user"),
    (empty_email(), ValidationError, "empty email"),
])
def test_login(input_data, expected_type, description):
    result = auth.login(input_data)
    assert isinstance(result, expected_type), f"Failed: {description}"
```

```typescript
// TypeScript with Vitest/Jest
describe('login', () => {
  const cases = [
    { input: validCredentials, expected: 'TokenPair', desc: 'valid credentials' },
    { input: wrongPassword, expected: 'AuthError', desc: 'invalid password' },
    { input: unknownEmail, expected: 'AuthError', desc: 'unknown user' },
  ]

  cases.forEach(({ input, expected, desc }) => {
    it(`returns ${expected} for ${desc}`, () => {
      const result = auth.login(input)
      expect(result.constructor.name).toBe(expected)
    })
  })
})
```

```go
// Go table-driven
func TestLogin(t *testing.T) {
    tests := []struct {
        name     string
        input    Credentials
        wantErr  bool
        errType  error
    }{
        {"valid credentials", validCreds, false, nil},
        {"invalid password", wrongPwd, true, ErrAuthFailed},
        {"unknown user", unknownUser, true, ErrUserNotFound},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            result, err := auth.Login(tt.input)
            if tt.wantErr {
                assert.ErrorIs(t, err, tt.errType)
            } else {
                assert.NoError(t, err)
                assert.NotNil(t, result)
            }
        })
    }
}
```

**When to use:** Multiple similar test cases, error type coverage, edge case enumeration.

### Pattern 2: Arrange-Act-Assert (Preferred for Complex Single Cases)

When a test has complex setup or multiple assertions:

```python
def test_login_creates_session_and_returns_tokens():
    # ARRANGE - Setup test data and dependencies
    user = create_test_user(email="test@example.com", password="secret123")
    credentials = Credentials(email="test@example.com", password="secret123")

    # ACT - Execute the function under test
    result = auth.login(credentials)

    # ASSERT - Verify all expected outcomes
    assert isinstance(result, TokenPair)
    assert result.access_token is not None
    assert result.refresh_token is not None

    # Verify session was created
    session = session_repo.get_by_user(user.id)
    assert session is not None
    assert session.is_active
```

**When to use:** Complex setup, multiple related assertions, integration tests.

### Pattern 3: Given-When-Then (For Behavior-Focused Tests)

When test names should clearly describe behavior:

```python
class TestLogin:
    def test_given_valid_credentials_when_login_then_returns_token_pair(self):
        # Given
        user = create_user(password="correct")
        credentials = Credentials(email=user.email, password="correct")

        # When
        result = auth.login(credentials)

        # Then
        assert isinstance(result, TokenPair)
        assert result.access_token is not None

    def test_given_wrong_password_when_login_then_returns_auth_error(self):
        # Given
        user = create_user(password="correct")
        credentials = Credentials(email=user.email, password="wrong")

        # When
        result = auth.login(credentials)

        # Then
        assert isinstance(result, AuthError)
        assert result.code == "INVALID_PASSWORD"
```

**When to use:** BDD-style projects, when test names should be readable as specifications.

### Pattern 4: Test Fixtures for Shared Setup

When multiple tests need similar setup:

```python
@pytest.fixture
def auth_service(test_db, user_repo, session_repo):
    """Provides configured AuthService for tests."""
    return AuthService(
        user_repo=user_repo,
        session_repo=session_repo,
        token_generator=JWTGenerator(secret="test-secret")
    )

@pytest.fixture
def valid_user(user_repo):
    """Creates a valid user for authentication tests."""
    user = User(email="test@example.com", password_hash=hash("password123"))
    user_repo.save(user)
    return user

def test_login_success(auth_service, valid_user):
    credentials = Credentials(email=valid_user.email, password="password123")
    result = auth_service.login(credentials)
    assert isinstance(result, TokenPair)

def test_login_wrong_password(auth_service, valid_user):
    credentials = Credentials(email=valid_user.email, password="wrong")
    result = auth_service.login(credentials)
    assert isinstance(result, AuthError)
```

**When to use:** Multiple tests share setup, DRY test code, complex dependencies.

### Choosing the Right Pattern

| Situation | Recommended Pattern |
|-----------|---------------------|
| Many similar input/output pairs | Table-Driven |
| Complex setup with single scenario | Arrange-Act-Assert |
| Behavior specifications | Given-When-Then |
| Shared setup across tests | Fixtures |
| Error case enumeration | Table-Driven |
| Integration with external services | Arrange-Act-Assert |

---

## Rule 7: Behavioral Assertions

Tests MUST include assertions that **force real implementation**. Type checks alone are insufficient.

→ See: `rules.md § Test Smells` for anti-patterns
→ See: `rules.md § Reality Checks` for verification questions

### Assertion Patterns by Action

| Action in `for:` | Required Assertions |
|------------------|---------------------|
| "Create X" | X exists in storage after call, X has correct fields, X.id is valid |
| "Update X" | X.field changed to new value, X.updated_at changed |
| "Delete X" | X no longer in storage, related data cleaned up |
| "Authenticate" | Token is valid JWT, token contains user_id, token not expired |
| "Validate X" | Invalid input rejected with specific error, valid input accepted |
| "Find/Get X" | Returned X matches stored X, all fields populated |
| "Transform X to Y" | Y has expected structure, Y values derived correctly from X |

## Rule 8: Anti-Fake Tests

Every function that could be faked MUST have tests that catch fakes.

→ See: `rules.md § Fakes vs Real` for fake vs real comparison
→ See: `rules.md § Reality Checks` for the 4 verification checks
→ See: `rules.md § When to Strengthen Tests` for test improvement patterns

### Summary of Anti-Fake Patterns

| Fake Type | Test Strategy |
|-----------|---------------|
| Hardcoded response | Different inputs → different outputs |
| In-memory storage | Data survives new service instance |
| No-op function | Verify side effects occurred |
| Fake external call | Verify real API behavior |

## Derivation Checklist

For each function in `provides:`:

- [ ] At least 1 happy path test from `for:`
- [ ] 1 error test per error type in `output:`
- [ ] Edge case tests for required inputs
- [ ] Null/empty handling test
- [ ] Behavioral assertions (→ `rules.md § Test Smells`)
- [ ] Anti-fake tests (→ `rules.md § Reality Checks`)

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
