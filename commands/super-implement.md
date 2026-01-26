---
description: Implement changes following industry best practices without backward compatibility concerns
---

# Super Implement

Implement the recommended changes with these principles:

1. **No Backward Compatibility**: Remove all legacy patterns, deprecated code, and compatibility shims. Update everything to the modern approach.

2. **Industry Best Practices**: Follow the best practices identified in the review. Every change should align with current standards.

3. **Clean Slate Approach**:
   - Delete deprecated code entirely (no `_unused` renames)
   - Remove compatibility layers
   - Update all call sites to new patterns
   - No "old way" comments or TODOs

4. **Consistency**: Apply changes uniformly across the codebase. Same pattern everywhere.

5. **Complete Implementation**: Don't leave partial updates. If changing a pattern, change it everywhere.
