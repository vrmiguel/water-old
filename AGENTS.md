# AGENTS.md

## Cursor Cloud specific instructions

This is a pure Rust library/binary crate (WAT compiler). No external services, databases, or containers needed.

### Key commands

| Action | Command |
|---|---|
| Build | `cargo build` |
| Test | `cargo test` |
| Run | `cargo run` |
| Lint | `cargo +stable clippy -- -D warnings` |
| Format check | `cargo +nightly fmt -- --check` |

### Notes

- Clippy uses the **stable** toolchain; rustfmt uses the **nightly** toolchain (required for `edition = "2021"` settings in `.rustfmt.toml` like `imports_granularity` and `group_imports`).
- The CI workflow (`.github/workflows/build-and-test.yml`) mirrors these commands. Pre-existing clippy lifetime-elision warnings exist in the codebase and cause `clippy -- -D warnings` to fail; these are not regressions.
- The sole dependency is `nom 7.1.1` (parser combinators). `cargo fetch` in the update script pre-downloads it.
