# Contributing

## Branch rules
- Do not push directly to `main`.
- Do work in a feature branch off `development`.
- Open a PR into `development` (or into `main` when preparing a release).

## Local dev commands
Run these before pushing:

- Format:
  - `cargo fmt --all`

- Lint:
  - `cargo clippy --all-targets --all-features -- -D warnings`

- Tests:
  - `cargo test --all-features`

## Commit style
Keep commits small and specific. Example:
- `Add inventory save/load`
- `Fix crash on empty input`
