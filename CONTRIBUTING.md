# Contributing to Tresca

Contributions are always welcome. 

## Before you contribute

Read [ARCHITECTURE.md](ARCHITECTURE.md). Most contribution discussions reduce to "how does this fit the layer model" and "is this consistent with the design principles." If you haven't read the architecture, the answer is "read this first."

For substantial changes, open an issue describing what you intend to do **before writing the code**. Partly to avoid wasted work, partly to surface design conflicts early.

## Code standards

- `cargo fmt` before every commit. CI enforces this.
- `cargo clippy --all-targets -- -D warnings` must pass.
- `cargo test --all` must pass.
- All public APIs require rustdoc comments — what they do, their invariants, non-obvious failure modes.
- New modules and significant functions should have brief design rationale in a comment, especially when the design choice is non-obvious.
- Avoid `unsafe` unless there's a measurable performance reason or FFI necessity. Justify every use in a comment.

## Commit messages

Conventional commits format:

- `feat(scope): description` for new features
- `fix(scope): description` for bug fixes
- `perf(scope): description` for performance improvements
- `refactor(scope): description` for code reorganization without behavior change
- `docs(scope): description` for documentation
- `chore(scope): description` for tooling, CI, configuration

Scopes: `core`, `render`, `physics`, `structural`, `app`, or a specific module path.

## Pull requests

Each PR should do *one* thing. Mixing a bug fix with a refactor and a new feature makes review impossible.

PR descriptions should include:

- What this changes
- Why (link the issue if applicable)
- How it was tested
- Any architectural implications

Reviewers will be candid. If a PR is wrong or the approach doesn't fit, expect direct feedback.

## What to work on

First-time contributors: pick an issue tagged `good-first-issue` or `help-wanted`. Scoped to be tractable without deep engine knowledge.

Experienced contributors: feature work is welcome on anything in [ROADMAP.md](ROADMAP.md). Performance work is always welcome. Documentation is genuinely valuable and goes through the same review process as code.

Out of scope (currently):

- Game-like features (HUDs, menus, multiplayer lobbies). Tresca is an engine. Build games or software separately, on top.
- Scenarios as engine-level features. Scenarios belong in user code.

## License of contributions

By submitting a contribution, you agree it is licensed under Apache 2.0, matching the project license. Sign commits with `git commit -s` to indicate Developer Certificate of Origin acceptance.
