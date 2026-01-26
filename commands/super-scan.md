---
description: Comprehensive scan to verify all updates are correct, consistent, and no legacy code remains
---

# Super Scan

Perform a comprehensive scan of all files related to the recent updates:

1. **Find All Related Files**: Identify every file that should have been affected by the changes.

2. **Consistency Check**: Verify the same patterns are used everywhere:
   - No mixed old/new approaches
   - Naming conventions consistent
   - Import patterns uniform
   - API usage consistent

3. **Legacy Code Detection**: Search for any remaining:
   - Old function/method names
   - Deprecated patterns
   - Compatibility shims
   - Dead code paths
   - Outdated comments referencing old behavior

4. **Conflict Detection**: Look for:
   - Type mismatches
   - Missing updates in dependent code
   - Broken references
   - Incomplete refactors

5. **Report**: Provide a summary of:
   - Files scanned
   - Issues found (if any)
   - Confirmation of completeness

If any issues are found, list them with file paths and line numbers for correction.
