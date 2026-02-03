---
name: visualize-spec
description: Generate Mermaid diagrams from spec.yaml to visualize system architecture, component dependencies, event flows, and type relationships.
user-invocable: true
---

# Visualize Spec

Generate comprehensive Mermaid diagrams from `spec.yaml` to help users understand the system design at a glance.

## Philosophy

- **Visual comprehension**: Complex specs become understandable through diagrams
- **Multiple perspectives**: Show architecture, dependencies, events, and types
- **Always current**: Regenerate anytime to reflect spec changes
- **Markdown-native**: Renders in GitHub, VSCode, Obsidian, and most markdown viewers

## Output

| File | Purpose |
|------|---------|
| `.opensdd/spec.visual.md` | Mermaid diagrams visualizing the spec |

## Diagrams Generated

| Diagram | Purpose |
|---------|---------|
| Architecture Overview | Components grouped by layer (domain/application/infrastructure) |
| Component Dependencies | Graph showing `consumes` relationships between components |
| Event Flow | How events flow from emitters to subscribers |
| Type Map | Types grouped by category with component usage |

## Start

<start>

### Step 1: Verify Prerequisites

```bash
test -f ".opensdd/spec.yaml" && echo "FOUND" || echo "NOT_FOUND"
```

- If `FOUND`: Proceed to Step 2
- If `NOT_FOUND`: Tell user "No spec.yaml found. Run `/opensdd:spec` first." and STOP workflow.

### Step 2: Load Spec

Read `.opensdd/spec.yaml` and parse its structure:
- `tech_stack` - Language, framework, database
- `structure.layers` - Layer definitions
- `types` - All type definitions with `for` and `used`
- `components` - Component contracts with provides, emits, subscribes, consumes
- `integrations` - External system connections

### Step 3: Generate Diagrams

Load `references/diagrams/mermaid-templates.md` and generate each diagram:

1. **Architecture Overview** - C4-style container diagram
2. **Component Dependencies** - Flowchart of consumes relationships
3. **Event Flow** - Event emit/subscribe flow
4. **Type Map** - Class diagram with usage links

### Step 4: Write Output

Write all diagrams to `.opensdd/spec.visual.md` with:
- Header with generation timestamp
- Table of contents
- Each diagram section with title and description
- Navigation links between sections

### Step 5: Report Success

Display summary:
- Number of components visualized
- Number of events mapped
- Number of types included
- Path to output file

</start>
