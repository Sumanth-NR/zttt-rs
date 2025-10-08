# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in zttt-rs, please report it by:

1. **Do NOT** open a public GitHub issue
2. Email the maintainers directly (see Cargo.toml for contact)
3. Include detailed information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a more detailed response within 5 business days.

## Security Considerations

While zttt-rs is a game library with limited attack surface, we take security seriously:

- **Input Validation**: All board positions are validated
- **No Unsafe Code**: The library uses only safe Rust
- **No External Dependencies**: Minimizes supply chain risks
- **Memory Safety**: Rust's ownership system prevents memory issues

## Best Practices for Users

When using zttt-rs:
- Validate user input before passing to the library
- Keep the library updated to the latest version
- Follow Rust security best practices in your application

## Disclosure Policy

- Security issues are handled privately until a fix is available
- Once fixed, we will publish a security advisory
- Credit will be given to reporters (unless they prefer to remain anonymous)
