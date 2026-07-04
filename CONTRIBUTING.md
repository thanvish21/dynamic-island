# Contributing to Dynamic Island

Thank you for your interest in contributing to Dynamic Island! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior by opening an issue.

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check the [existing issues](https://github.com/thanvish21/dynamic-island/issues) to avoid duplicates.

When filing a bug report, include:
- Your operating system and version
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Screenshots or screen recordings if applicable
- Any relevant log output from the terminal

### Suggesting Features

Feature requests are welcome! Please open an issue with the **feature request** template and describe:
- The problem your feature would solve
- Your proposed solution
- Any alternatives you've considered

### Pull Requests

1. **Fork** the repository and create your branch from `main`
2. **Install** the development environment (see below)
3. **Make** your changes in a dedicated feature branch
4. **Test** your changes thoroughly on your platform
5. **Commit** with clear, descriptive commit messages
6. **Push** your branch and open a Pull Request

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [pnpm](https://pnpm.io/) 8+

### Linux Dependencies

```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev librsvg2-dev patchelf

# Fedora
sudo dnf install webkit2gtk4.1-devel librsvg2-devel

# Arch
sudo pacman -S webkit2gtk-4.1 librsvg
```

### Getting Started

```bash
# Fork and clone
git clone https://github.com/<your-username>/dynamic-island.git
cd dynamic-island

# Install dependencies
pnpm install

# Start development server
pnpm tauri dev
```

### Project Structure

- `src/` -- Svelte 5 frontend (components, stores, utilities)
- `src-tauri/` -- Rust backend (system APIs, Tauri commands)
- `.github/` -- CI/CD workflows and issue templates

## Code Style

### Frontend (Svelte / TypeScript)

- Use **Svelte 5** runes (`$state`, `$derived`, `$effect`) instead of legacy stores where possible
- Use TypeScript for all new files
- Follow existing component patterns and naming conventions
- Keep components focused -- one responsibility per component
- Use CSS custom properties for theming values

### Backend (Rust)

- Follow standard Rust conventions (`cargo fmt`, `cargo clippy`)
- Use descriptive variable and function names
- Handle errors properly with `Result` types -- no `unwrap()` in production code
- Document public functions with doc comments
- Keep Tauri commands thin -- delegate logic to dedicated modules

### General

- Write clear, descriptive commit messages
- Keep PRs focused -- one feature or fix per PR
- Add comments only where the logic is non-obvious
- Remove debug/console logging before submitting

## Commit Messages

Use clear, imperative commit messages:

```
Add media widget progress bar animation
Fix CPU usage calculation on Windows
Update weather API to use Open-Meteo v2
Remove unused notification store imports
```

## Testing

- Test your changes on your platform before submitting
- If you add a new Tauri command, verify it works on both Linux and Windows
- Test edge cases (no network, no media player running, etc.)

## Review Process

1. A maintainer will review your PR within a few days
2. Address any requested changes
3. Once approved, your PR will be merged into `main`

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

Thank you for helping make Dynamic Island better!
