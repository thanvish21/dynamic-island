# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| Latest  | Yes                |
| < 1.0   | Development only   |

## Reporting a Vulnerability

If you discover a security vulnerability in Dynamic Island, please report it responsibly.

**Do NOT open a public issue for security vulnerabilities.**

Instead, please email or contact the maintainer directly:

- GitHub: [@thanvish21](https://github.com/thanvish21)

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Acknowledgement**: Within 48 hours
- **Assessment**: Within 1 week
- **Fix & Release**: As soon as possible, depending on severity

### Scope

This security policy covers:

- The Tauri application and its Rust backend
- The Svelte frontend code
- Build and CI/CD configurations
- Any bundled dependencies

### Out of Scope

- Third-party services (Open-Meteo API, MPRIS)
- Operating system vulnerabilities
- Issues in upstream dependencies (report these to the respective projects)

## Security Best Practices

When contributing, please ensure:

- No hardcoded secrets or API keys in source code
- Proper input validation for any user-provided data
- Use of Tauri's permission system for system API access
- Dependencies are kept up to date

Thank you for helping keep Dynamic Island and its users safe.
