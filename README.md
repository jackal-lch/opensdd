<p align="center">
  <img src="docs/assets/banner-v2.png" alt="OpenSDD - Spec-Driven Development that actually works" width="800">
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://claude.ai/code"><img src="https://img.shields.io/badge/Claude%20Code-Plugin-blueviolet" alt="Claude Code Plugin"></a>
</p>

---

## Quick Start

**1. Open Claude Code**
```bash
claude
```

**2. Add the marketplace**
```
/plugin marketplace add jackal-lch/opensdd
```

**3. Install the plugin**
```
/plugin install opensdd@opensdd-marketplace
```

**4. Install spec-extract** (in a separate terminal)
```bash
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash
```

**5. Start building**
```
/opensdd:blueprint
```

---

## Table of Contents

- [Why OpenSDD?](#why-opensdd)
- [How It Works](#how-it-works)
- [Commands](#commands)
- [Installation](#installation)
- [Example Session](#example-session)
- [Supported Languages](#supported-languages)
- [Contributing](#contributing)
- [License](#license)

---

## Why OpenSDD?

Traditional specs become fiction:

```
Day 1:  Spec is perfect
Day 30: Spec is stale
Day 60: Nobody trusts it
```

**OpenSDD fixes this.** It continuously verifies that your code matches the spec. When they drift apart, the code gets fixed automatically.

The spec is the source of truth. Always.

---

## How It Works

```
/opensdd:blueprint  →  /opensdd:spec  →  /opensdd:visualize  →  /opensdd:build
```

| Command | What It Does |
|---------|--------------|
| `/opensdd:blueprint` | Define your product (vision, users, features, flows) |
| `/opensdd:spec` | Generate technical contracts (components, types, interfaces) |
| `/opensdd:visualize` | Generate Mermaid diagrams to understand system design |
| `/opensdd:build` | Implement and verify code matches spec |
| `/opensdd:compare` | Check code-spec alignment anytime (standalone) |

The build loop runs continuously:

```
┌──────────────────────────────────────────┐
│                                          │
│  Implement → Extract → Compare → Fix     │
│      ↑                           │       │
│      └───────────────────────────┘       │
│                                          │
└──────────────────────────────────────────┘
```

1. **Implement** — AI writes code from your spec
2. **Extract** — `spec-extract` pulls signatures from code
3. **Compare** — AI semantically compares code vs spec
4. **Fix** — Drift is automatically corrected

---

## Commands

| Command | Purpose |
|---------|---------|
| `/opensdd:blueprint` | 8-phase guided product definition |
| `/opensdd:spec` | 4-phase technical specification |
| `/opensdd:visualize` | Generate Mermaid diagrams from spec |
| `/opensdd:build` | Implement, verify, and fix loop |
| `/opensdd:compare` | Check code-spec alignment (matches, drifts, missing, extras) |
| `/opensdd:cov` | Chain of verification for response validation |

---

## Installation

### Prerequisites

- [Claude Code](https://claude.ai/code) CLI installed
- macOS, Linux, or Windows

### Step 1: Install the Plugin

Open Claude Code and run these commands:

```
claude
```

Then inside Claude Code:

```
/plugin marketplace add jackal-lch/opensdd
/plugin install opensdd
```

To verify installation:

```
/opensdd:
```

You should see `blueprint`, `spec`, `visualize`, `build`, and `cov` in the autocomplete.

### Step 2: Install spec-extract

The `spec-extract` tool extracts code signatures for verification.

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.ps1 | iex
```

### Step 3: Verify Installation

```bash
spec-extract --version
```

---

## Example Session

```
You: I want to build a task management CLI

> /opensdd:blueprint
[Interactive: vision, users, features, flows...]
✓ Created .opensdd/blueprint.md

> /opensdd:spec
[Interactive: components, types, contracts...]
✓ Created .opensdd/spec.yaml

> /opensdd:build
Phase 1: Select → TaskManager component
Phase 2: Implement → AI writes code
Phase 3: Verify → Extract and compare
  ⚠ Drift: createTask vs new_task
  → Fixed: renamed to create_task
  ✓ Code matches spec
Phase 4: Review → 1 helper (auto-kept)
✓ Complete
```

**Generated project structure:**

```
your-project/
├── .opensdd/
│   ├── blueprint.md       # Product definition
│   ├── spec.yaml          # Technical contracts (source of truth)
│   ├── spec-visual.md     # Mermaid diagrams (via /visualize)
│   ├── compare-result.yaml # Code-spec alignment report (via /compare)
│   └── extracted/         # Code signatures
└── src/
    └── ...                # Your code, verified against spec
```

---

## Supported Languages

| Language | Status |
|----------|--------|
| TypeScript | Full support |
| Python | Full support |
| Rust | Full support |
| Go | Full support |

---

## Contributing

Contributions are welcome! Here's how to get started:

### Development Setup

```bash
git clone https://github.com/jackal-lch/opensdd.git
cd opensdd

# Test the plugin locally
claude --plugin-dir .
```

### Building spec-extract

```bash
cd tools/spec-extract
cargo build --release
```

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Push to your fork
5. Open a Pull Request

### Reporting Issues

Found a bug or have a feature request? [Open an issue](https://github.com/jackal-lch/opensdd/issues).

---

## License

[MIT](LICENSE)
