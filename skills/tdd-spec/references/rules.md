# TDD Rules

Single source of truth for all TDD rules. Phase files reference these by section name.

---

## § Absolute Rule

```
IF passing a test requires a fake or placeholder:
  1. STOP — Do not add the fake
  2. The TEST is too weak
  3. Strengthen the TEST first
  4. Then implement REAL logic
```

**Never fake it to make it green. Fix the test instead.**

---

## § Fakes vs Real

| Fake (WRONG) | Real (CORRECT) |
|--------------|----------------|
| `return "fake_token"` | `return generate_jwt(user_id)` |
| `_users = {}` (in-memory dict) | `self.db.save(user)` |
| `return Mock()` | Use actual dependency |
| `pass` or `raise NotImplementedError` | Implement the logic |
| Hardcoded value matching test | Computed value from real logic |

**"Minimal implementation" = No over-engineering. NOT fakes/stubs.**

---

## § Reality Checks

Before marking a function GREEN, verify implementation is REAL:

| # | Check | Question | Red Flag |
|---|-------|----------|----------|
| 1 | **Computation** | Different inputs → different outputs? | Hardcoded return value |
| 2 | **Persistence** | Data retrievable in new session? | In-memory dict |
| 3 | **External** | Real SDK/API call made? | Hardcoded response |
| 4 | **Fields** | All type fields have real values? | Empty/placeholder fields |

**If any check fails:** Test is too weak → Strengthen test → Re-implement

### Reality Check Examples

**Check 1 - Computation:** Prove different inputs produce different outputs

```python
def test_not_hardcoded():
    result1 = service.process(Input(value="hello"))
    result2 = service.process(Input(value="world"))
    assert result1.output != result2.output  # Fails if hardcoded!
```

**Check 2 - Persistence:** Prove data survives new service instance

```python
def test_persists_to_database():
    # Create with one instance
    service1 = UserService(db=db)
    user = service1.create(CreateUserInput(email="test@example.com"))

    # Retrieve with NEW instance (proves not in-memory dict)
    service2 = UserService(db=db)
    found = service2.get_by_id(user.id)

    assert found is not None  # Fails if _users = {} in-memory!
    assert found.email == "test@example.com"
```

**Check 3 - External:** Prove real SDK/API call made

```python
def test_real_external_call():
    service = ChatService(sdk=real_sdk_client)
    result = service.invoke(AgentInput(prompt="What is 2+2?"))

    # Real API gives meaningful response
    assert result.response is not None
    assert len(result.response) > 0
    assert result.response != "fake response"  # Catch hardcoded!
    assert result.tokens_used > 0  # Real API returns metrics
```

**Check 4 - Fields:** Prove type fields are populated

```python
def test_returns_complete_type():
    result = auth.login(credentials)

    # All fields must have real values
    assert result.access_token is not None
    assert len(result.access_token) > 20  # Real JWT is long
    assert result.refresh_token is not None
    assert result.expires_in > 0
    assert result.token_type == "Bearer"
```

---

## § Verification Questions

### RED Phase Verification

After running tests (expecting failure):

| Question | Expected | If Not |
|----------|----------|--------|
| Did tests FAIL? | YES | Investigate — may already be implemented |
| Failure reason = "not implemented"? | YES | Fix syntax/import error |
| Any unexpected PASSES? | NO | Tests may be wrong |

**Proceed only when:** All three conditions met.

### GREEN Phase Verification

After running tests (expecting pass):

| Question | Expected | If Not |
|----------|----------|--------|
| Did ALL tests PASS? | YES | Fix implementation, re-run |
| Passed count = Total count? | YES | Some tests may be skipped |
| Any regressions in other functions? | NO | You broke something |

**Proceed only when:** All three conditions met.

---

## § Test Smells

Anti-patterns that indicate weak tests:

### Smell 1: Testing only types

```python
# WEAK — Fakes pass this
assert isinstance(result, TokenPair)

# STRONG — Forces real implementation
assert result.access_token is not None
assert len(result.access_token) > 20
assert decode_jwt(result.access_token)["user_id"] == user.id
```

### Smell 2: Same input everywhere

```python
# WEAK — Doesn't prove computation
def test_hash(): assert hash("password") == "abc123"

# STRONG — Proves real hashing
def test_hash():
    assert hash("password1") != hash("password2")
```

### Smell 3: No persistence verification

```python
# WEAK — In-memory dict passes
user = service.create(data)
assert user.id is not None

# STRONG — Proves database persistence
user = service.create(data)
stored = db.query(User).get(user.id)
assert stored.email == data.email
```

### Smell 4: Tests depend on each other

```python
# WEAK — Order-dependent
def test_create(): service.create(data)
def test_read(): service.get(data.id)  # Needs create!

# STRONG — Independent
def test_read():
    user = create_test_user()  # Own setup
    assert service.get(user.id) == user
```

### Smell 5: No side effect verification

```python
# WEAK — No-op function passes
def test_record_metric():
    service.record_metric("count", 1.0)
    # No assertion = function can be empty!

# STRONG — Verifies side effect occurred
def test_record_metric():
    service.record_metric("count", 1.0)
    value = registry.get_sample_value("count")
    assert value == 1.0  # Fails if no-op!
```

### Smell 6: Mocking what should be real

```python
# WEAK — Mocks hide missing implementation
def test_with_mock():
    mock_repo = Mock()
    mock_repo.save.return_value = User(id="123")
    service = UserService(repo=mock_repo)
    result = service.create(data)
    assert result.id == "123"  # Proves nothing!

# STRONG — Uses real (test) repository
def test_with_real_repo():
    repo = TestRepository(db=test_db)
    service = UserService(repo=repo)
    result = service.create(data)
    stored = repo.get_by_id(result.id)
    assert stored.email == data.email  # Proves persistence!
```

---

## § Implementation Requirements

When implementing a function:

1. **Achieve the purpose** — `for:` says "Create user" → actually persist
2. **Define type fields** — Return `TokenPair` → define its fields
3. **Use real dependencies** — Call actual repos/services/SDKs
4. **Handle errors** — Return correct error types

### Self-Check (ask for each line of code)

- "Could this be a fake?" → If yes, make it real
- "Does this actually persist/compute/call?" → If no, fix it

---

## § When to Strengthen Tests

A test needs strengthening when:

| Situation | Test Fix |
|-----------|----------|
| Can pass with hardcoded value | Assert different inputs → different outputs |
| Can pass with in-memory dict | Assert data survives new service instance |
| Can pass with empty function | Assert side effects (metrics, logs, state) |
| Only checks `isinstance()` | Assert specific field values |
| Doesn't verify persistence | Query database after operation |

---

## § Not Implementable Detection

A function is **not implementable** when the spec/blueprint lacks information required for real implementation.

### Signs of Missing Information

| Category | Missing Info | Example |
|----------|--------------|---------|
| **External Service** | SDK methods, API endpoints, auth | "Call payment API" but no SDK/endpoint defined |
| **Business Logic** | Rules, formulas, algorithms | "Calculate price" but no pricing rules |
| **Data Schema** | Field mappings, transformations | "Transform to X format" but format undefined |
| **Dependencies** | Other components not yet built | "Use UserService" but UserService not implemented |
| **Configuration** | Environment vars, secrets | "Use API key" but no config structure |

### Detection Questions

For each function, ask:

1. **Can I write the implementation body?**
   - If NO → What information is missing?

2. **Is the missing info in spec.yaml?**
   - If NO → Function is NOT IMPLEMENTABLE

3. **Is the missing info in referenced docs/SDKs?**
   - If NO → Function is NOT IMPLEMENTABLE

### When Detected

```
IF function is NOT IMPLEMENTABLE:
  1. Do NOT create fake/placeholder implementation
  2. Do NOT write tests that would require fakes
  3. Mark function as BLOCKED
  4. Document what information is missing
  5. Continue with other functions
  6. Report blocked functions at end
```

### Blocked Function Record

```yaml
blocked_functions:
  - name: invokeAgent
    reason: "SDK method signatures not defined in spec"
    missing:
      - "HiAgentSDK.invoke() parameters"
      - "AgentResponse structure"
    suggested_fix: "Add SDK documentation or mock interface to spec"
```

### NOT Blocked (Can Still Implement)

| Situation | Why Implementable |
|-----------|-------------------|
| Uses only domain types | Types defined in spec |
| Pure computation | Algorithm can be derived from description |
| CRUD with defined schema | Repository pattern + types known |
| Validation rules stated | Rules explicit in spec |
| Transformation with examples | Input/output examples provided |

---

## § Refactoring Rules

During REFACTOR phase:

**DO:**
- Remove duplication
- Improve naming
- Extract helpers (if clearly beneficial)
- Simplify conditionals

**DON'T:**
- Add new features
- Change behavior
- Gold-plate

**Discipline:** Run tests after each change. If fail → undo → try different approach.
