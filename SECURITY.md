# Security Policy

## Reporting a vulnerability

If you discover a security vulnerability in Tresca, **do not file a public issue**. Email [joegilliard2@gmail.com] with details, or use GitHub's private vulnerability reporting feature.

Reports should include:

- The version or commit hash affected
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested mitigation if available

Confirmed vulnerabilities are patched as quickly as practical, with credit given to the reporter unless they prefer otherwise.

## Scope

Tresca is a Rust crate. The security surface includes:

- Memory safety violations in unsafe code
- Denial-of-service vectors via malformed input data
- Unsafe handling of user-supplied scene or material definitions
- Vulnerabilities in dependency configuration affecting downstream users

The engine does not handle network traffic, authentication, or persistent user data in v0.1. As features extend into those areas, this policy will be updated.

## Out of scope

- Bugs that cause incorrect physics or rendering without security implications. Report as regular issues.
- Performance issues. Report as regular issues.
- Adversarial use of the engine for content that is legal but objectionable. Tresca is a simulation tool; the user is responsible for the content of their simulations within applicable law.

## Disclosure

Coordinated disclosure preferred. Vulnerabilities are disclosed publicly after a patch is available, with a brief description of the issue, the fix, and reporter credit. The typical window between patch and disclosure is 7 days, longer for severe issues requiring coordinated rollout.
