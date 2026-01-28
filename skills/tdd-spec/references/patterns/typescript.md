# TypeScript Test Patterns

Test patterns for TypeScript projects using Vitest or Jest.

## Framework Detection

```bash
# Check package.json for test framework
cat package.json | grep -E '"(vitest|jest)"'
```

- Prefer Vitest if both present
- Default to Vitest for new projects

## Imports

### Vitest
```typescript
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
```

### Jest
```typescript
import { describe, it, expect, jest, beforeEach, afterEach } from '@jest/globals'
```

### Component & Types
```typescript
// Component under test (will fail until implemented)
import { ComponentName } from '../{layer}/{component-file}'

// Types from shared types
import type { InputType, OutputType, ErrorType } from '../types'
```

## Test Structure

```typescript
describe('ComponentName', () => {
  // Setup
  let component: ComponentName

  beforeEach(() => {
    component = new ComponentName()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  describe('functionName', () => {
    // Happy path tests
    it('describes expected behavior', () => {
      // Arrange
      const input: InputType = { /* test data */ }

      // Act
      const result = component.functionName(input)

      // Assert
      expect(result).toBeInstanceOf(OutputType)
    })

    // Error case tests
    it('returns ErrorType when condition', () => {
      // Arrange
      const input: InputType = { /* invalid data */ }

      // Act
      const result = component.functionName(input)

      // Assert
      expect(result).toBeInstanceOf(ErrorType)
      expect(result.code).toBe('EXPECTED_ERROR_CODE')
    })

    // Edge case tests
    it('handles null input', () => {
      expect(() => component.functionName(null as any)).toThrow()
    })

    it('handles empty input', () => {
      const input: InputType = { /* empty values */ }
      const result = component.functionName(input)
      expect(result).toBeInstanceOf(ErrorType)
    })
  })
})
```

## Assertions

```typescript
// Equality
expect(value).toBe(expected)           // Strict equality
expect(value).toEqual(expected)        // Deep equality
expect(value).toStrictEqual(expected)  // Deep + type equality

// Type checking
expect(value).toBeInstanceOf(ClassName)
expect(value).toBeDefined()
expect(value).toBeNull()
expect(value).toBeTruthy()

// Errors
expect(() => fn()).toThrow()
expect(() => fn()).toThrow(ErrorType)
expect(() => fn()).toThrowError('message')

// Async
await expect(asyncFn()).resolves.toBe(expected)
await expect(asyncFn()).rejects.toThrow()

// Arrays
expect(array).toContain(item)
expect(array).toHaveLength(n)

// Objects
expect(obj).toHaveProperty('key', value)
expect(obj).toMatchObject({ partial: 'match' })
```

## Mocking

### Vitest
```typescript
// Mock a module
vi.mock('../path/to/module', () => ({
  functionName: vi.fn(() => mockReturnValue)
}))

// Mock a method
const spy = vi.spyOn(object, 'method')
spy.mockReturnValue(mockValue)
spy.mockResolvedValue(asyncMockValue)

// Verify calls
expect(spy).toHaveBeenCalled()
expect(spy).toHaveBeenCalledWith(arg1, arg2)
expect(spy).toHaveBeenCalledTimes(n)
```

### Jest
```typescript
// Mock a module
jest.mock('../path/to/module', () => ({
  functionName: jest.fn(() => mockReturnValue)
}))

// Mock a method
const spy = jest.spyOn(object, 'method')
spy.mockReturnValue(mockValue)
```

## Event Testing

```typescript
describe('events', () => {
  it('emits EventName on condition', () => {
    // Arrange
    const eventHandler = vi.fn()
    component.on('EventName', eventHandler)

    // Act
    component.triggerAction()

    // Assert
    expect(eventHandler).toHaveBeenCalledWith(
      expect.objectContaining({
        expectedField: expect.any(String)
      })
    )
  })
})
```

## Async Testing

```typescript
describe('async operations', () => {
  it('resolves with expected value', async () => {
    const result = await component.asyncFunction(input)
    expect(result).toBe(expected)
  })

  it('rejects with error', async () => {
    await expect(component.asyncFunction(badInput))
      .rejects
      .toThrow(ErrorType)
  })
})
```

## Test Data Factories

```typescript
// Create test data factories for common types
function createCredentials(overrides?: Partial<Credentials>): Credentials {
  return {
    email: 'test@example.com',
    password: 'TestPassword123',
    ...overrides
  }
}

// Usage
it('authenticates valid credentials', () => {
  const credentials = createCredentials()
  const result = authService.login(credentials)
  expect(result).toBeInstanceOf(AuthResult)
})

it('rejects invalid email', () => {
  const credentials = createCredentials({ email: 'invalid' })
  const result = authService.login(credentials)
  expect(result).toBeInstanceOf(AuthError)
})
```
