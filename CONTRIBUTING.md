# Contributing to Haven

Thank you for your interest in contributing to Haven! We're building the decentralized standard for device registries on Stellar, and every contribution matters.

## 🚀 Getting Started

### 1. Fork & Clone

```bash
# For contracts
git clone https://github.com/Haven-Protocol/haven-contracts.git

# For frontend
git clone https://github.com/Haven-Protocol/haven-frontend.git
```

### 2. Pick an Issue

- Browse open issues on [haven-contracts](https://github.com/Haven-Protocol/haven-contracts/issues) or [haven-frontend](https://github.com/Haven-Protocol/haven-frontend/issues)
- Look for labels: `good-first-issue`, `help-wanted`, `contracts`, `frontend`
- Comment on the issue to let us know you're working on it

### 3. Find `TODO` Markers

The codebase is intentionally full of `// TODO:` comments marking areas that need implementation. Run:

```bash
# Find all TODOs in contracts
grep -rn "TODO" contracts/

# Find all TODOs in frontend
grep -rn "TODO" frontend/src/
```

---

## 🌿 Branch Naming

Use descriptive branch names:

```
feature/bounty-escrow-logic
fix/device-state-validation
docs/update-readme
```

---

## 💻 Development Workflow

### Smart Contracts (Rust/Soroban)

```bash
cd contracts

# Check compilation
cargo check

# Run tests
cargo test

# Lint
cargo fmt --check
cargo clippy -- -D warnings

# Build WASM
stellar contract build
```

### Frontend (Next.js)

```bash
cd frontend

# Install dependencies
npm install

# Run dev server
npm run dev

# Lint
npm run lint

# Build
npm run build
```

---

## 📝 Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(contracts): implement bounty deposit logic
fix(frontend): resolve dark mode toggle flicker
docs: update architecture diagram in README
test(contracts): add recovery payout test
chore: update soroban-sdk dependency
```

---

## 🔀 Pull Request Process

1. **Create a branch** from `main`
2. **Make your changes** with clear, well-tested code
3. **Run all checks** (formatting, linting, tests)
4. **Submit a PR** with:
   - A clear title following conventional commits
   - A description of what changed and why
   - Reference to the related issue (e.g., `Closes #12`)
5. **Wait for review** — maintainers will review within 48 hours

---

## 🏷️ Issue Labels

| Label | Description |
|-------|-------------|
| `good-first-issue` | Perfect for newcomers to the project |
| `help-wanted` | We need community help on this |
| `contracts` | Related to Soroban smart contracts |
| `frontend` | Related to the Next.js frontend |
| `documentation` | Improvements to docs or README |
| `bug` | Something isn't working |
| `enhancement` | New feature or improvement |

---

## 🎨 Code Style

### Rust
- Format with `cargo fmt`
- Lint with `cargo clippy`
- Use `#![no_std]` in all contract code
- Keep functions small and well-documented

### TypeScript / React
- ESLint config is provided in the project
- Use functional components with hooks
- Prefer `const` over `let`
- Use TypeScript strict mode

---

## 🤝 Code of Conduct

Be respectful, inclusive, and constructive. We're building technology to fight device theft and protect communities — let's build a community that reflects those values.

---

## 📬 Questions?

Open a [Discussion](https://github.com/Haven-Protocol/haven/discussions) or reach out to the maintainers. We're happy to help you get started!
