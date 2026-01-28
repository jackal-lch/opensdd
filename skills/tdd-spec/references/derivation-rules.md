# Test Derivation Rules

Rules for transforming spec.yaml definitions into test cases.

## Core Principle: Behavioral Tests

**Tests MUST verify actual behavior, not just return types.**

```python
# BAD - Passes with placeholder stub
def test_login():
    result = auth.login(credentials)
    assert isinstance(result, TokenPair)  # Placeholder returns TokenPair too!

# GOOD - Forces real implementation
def test_login_authenticates_valid_credentials():
    result = auth.login(valid_credentials)
    assert result.access_token is not None
    assert len(result.access_token) > 20  # Real JWT
    decoded = decode_jwt(result.access_token)
    assert decoded["user_id"] == expected_user.id
    assert decoded["exp"] > time.time()  # Not expired
```

**Key insight:** If a test can pass with a hardcoded stub, the test is too weak.

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

## Rule 7: Behavioral Assertions (CRITICAL)

Tests MUST include assertions that **force real implementation**. Type checks alone are insufficient.

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

### Storage Verification

For any function that modifies state:

```python
# GOOD - Verifies actual storage (not in-memory dict!)
def test_create_user():
    # Use real database session, not mock
    db = get_test_database()
    service = UserService(db=db)

    result = service.create(input)

    # Verify returned object
    assert result.id is not None
    assert result.email == input.email

    # Verify actually persisted to DATABASE (not in-memory)
    stored = db.query(User).filter_by(id=result.id).first()
    assert stored is not None
    assert stored.email == input.email

    # Verify survives new session (proves real persistence)
    db.close()
    new_db = get_test_database()
    reloaded = new_db.query(User).filter_by(id=result.id).first()
    assert reloaded is not None  # Would fail if in-memory dict!
```

### External Service Verification

For functions that call external services (SDK, API):

```python
# GOOD - Verifies actual external call
def test_invoke_agent():
    # Use real SDK client (or integration test client)
    sdk = HiAgentSDK(api_key=test_key)
    service = ChatService(sdk=sdk)

    result = service.invoke(AgentInput(prompt="Hello"))

    # Verify response is from real API, not hardcoded
    assert result.response is not None
    assert len(result.response) > 0
    assert result.response != "fake response"  # Catch hardcoded!
    assert result.response != "stub"
    assert "error" not in result.response.lower() or result.is_error

    # Verify response structure from real API
    assert result.tokens_used > 0  # Real API returns token count
    assert result.model is not None  # Real API returns model name
```

### Metrics/Observability Verification

For functions that record metrics:

```python
# GOOD - Verifies metrics actually recorded
def test_record_metric():
    # Use real prometheus registry
    registry = CollectorRegistry()
    service = MetricsService(registry=registry)

    service.record_metric("request_count", 1.0)

    # Verify metric was actually recorded
    metrics = list(registry.collect())
    assert any(m.name == "request_count" for m in metrics)

    # Verify value
    request_count = registry.get_sample_value("request_count")
    assert request_count == 1.0  # Would fail if no-op!
```

### Field Verification

Tests must verify type fields are populated correctly:

```python
# GOOD - Forces type fields to be implemented
def test_login_returns_complete_token_pair():
    result = auth.login(credentials)

    # TokenPair must have these fields (forces type definition)
    assert result.access_token is not None
    assert result.refresh_token is not None
    assert result.expires_in > 0
    assert result.token_type == "Bearer"
```

### Relationship Verification

For functions involving relationships:

```python
# GOOD - Verifies relationships work
def test_create_session_links_to_user():
    session = auth.create_session(user_id)

    # Session linked to user
    assert session.user_id == user_id

    # Can retrieve user's sessions
    user_sessions = sessions.get_by_user(user_id)
    assert session.id in [s.id for s in user_sessions]
```

## Rule 8: Anti-Fake Tests (CRITICAL)

Every function that could be faked MUST have tests that catch fakes.

### Detect Hardcoded Responses

```python
# For any function that returns data:
def test_invoke_not_hardcoded():
    # Call with different inputs
    result1 = service.invoke(AgentInput(prompt="Hello"))
    result2 = service.invoke(AgentInput(prompt="Goodbye"))

    # Different inputs MUST produce different outputs
    # (catches hardcoded return values)
    assert result1.response != result2.response
```

### Detect In-Memory Storage

```python
# For any CRUD function:
def test_create_persists_to_database():
    # Create in one service instance
    service1 = UserService(db=db)
    user = service1.create(CreateUserInput(email="test@example.com"))

    # Retrieve from NEW service instance
    service2 = UserService(db=db)
    found = service2.get_by_id(user.id)

    # Must find it (fails if in-memory dict!)
    assert found is not None
    assert found.email == "test@example.com"
```

### Detect No-Op Functions

```python
# For any function with side effects:
def test_record_metric_has_effect():
    service = MetricsService(registry=registry)

    # Record a metric
    service.record_metric("test_metric", 42.0)

    # Verify the side effect occurred
    value = registry.get_sample_value("test_metric")
    assert value == 42.0  # Fails if no-op!
```

### Detect Fake External Calls

```python
# For any function that calls external API:
def test_sdk_call_is_real():
    service = ChatService(sdk=real_sdk_client)

    result = service.invoke(AgentInput(prompt="What is 2+2?"))

    # Real API would give meaningful response
    assert "4" in result.response or "four" in result.response.lower()
    # Fake would return generic "fake response"
```

## Derivation Checklist

For each function in `provides:`:

- [ ] At least 1 happy path test from `for:`
- [ ] 1 error test per error type in `output:`
- [ ] Edge case tests for required inputs
- [ ] Null/empty handling test
- [ ] **Behavioral assertions that force real implementation**
- [ ] **Storage verification for state-changing functions**
- [ ] **Field verification for returned types**
- [ ] **Anti-fake test if function could be stubbed**
- [ ] **Different inputs produce different outputs test**
- [ ] **Persistence survives new instance test (for CRUD)**

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
