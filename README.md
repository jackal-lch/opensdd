<p align="center">
  <img src="docs/assets/banner.png" alt="OpenSDD - Spec-Driven Development that actually works" width="800">
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://claude.ai/code"><img src="https://img.shields.io/badge/Claude%20Code-Plugin-blueviolet" alt="Claude Code Plugin"></a>
</p>

---

## Quick Start

```bash
# 1. Add the plugin
claude /plugin marketplace add jackal-lch/opensdd
claude /plugin install opensdd

# 2. Install the spec-extract tool
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash

# 3. Start building
claude
> /create-blueprint
```

---

## Table of Contents

- [Why OpenSDD?](#why-opensdd)
- [How It Works](#how-it-works)
- [Skills](#skills)
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
/create-blueprint  →  /create-spec  →  /build-loop
```

| Skill | What It Does |
|-------|--------------|
| `/create-blueprint` | Define your product (vision, users, features, flows) |
| `/create-spec` | Generate technical contracts (components, types, interfaces) |
| `/build-loop` | Implement and verify code matches spec |

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

## Skills

| Skill | Command | Purpose |
|-------|---------|---------|
| Create Blueprint | `/create-blueprint` | 8-phase guided product definition |
| Create Spec | `/create-spec` | 4-phase technical specification |
| Build Loop | `/build-loop` | Implement, verify, and fix loop |

---

## Installation

### Prerequisites

- [Claude Code](https://claude.ai/code) CLI installed
- macOS, Linux, or Windows

### Step 1: Install the Plugin

```bash
claude /plugin marketplace add jackal-lch/opensdd
claude /plugin install opensdd
```

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

> /create-blueprint
[Interactive: vision, users, features, flows...]
✓ Created .opensdd/blueprint.md

> /create-spec
[Interactive: components, types, contracts...]
✓ Created .opensdd/spec.yaml

> /build-loop
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
│   ├── blueprint.md    # Product definition
│   ├── spec.yaml       # Technical contracts (source of truth)
│   └── extracted/      # Code signatures
└── src/
    └── ...             # Your code, verified against spec
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
