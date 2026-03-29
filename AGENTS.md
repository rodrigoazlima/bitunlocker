# Cline Rules for Rust Project - bitunlocker

## RULE: NO DEAD CODE
- All functions must be called or exported. Unused: remove it. Do not use `#[allow(dead_code)]`.
- Tests allowed only under `#[cfg(test)]`.
- Modules in `lib.rs`/`main.rs` must export at least one used item.
- All imports must be used (`_` prefix for intentional unused).

## RULE: RUST BEST PRACTICES
- **Error Handling**: Never use `unwrap()`/`expect()` without comment. Prefer `?`, `match`, `if let`.
- **Organization**: One responsibility per module. No duplicated logic. Share via `lib.rs` re-exports.
- **Naming**: `snake_case` for fn/var/mod, `PascalCase` for struct/enum/trait.
- **Tests**: `test_<function>_<scenario>`.
- **Docs**: `///` for all public items, starting with capital letter.

## RULE: RUST TESTING BEST PRACTICES
- Unit tests inside `#[cfg(test)] mod tests {}` in the same file.
- Use clear test names: `test_<function>_<scenario>`.
- Prefer `assert_eq!`, `assert!`, `assert_ne!`.
- Never use `unwrap()` in tests (use `expect("test message")` or proper handling).
- Integration tests go in `tests/` directory.
- Favor table-driven tests for multiple cases.
- Use `#[should_panic]` sparingly and only with expected message when needed.

## RULE: CODE QUALITY
- Refactor any duplicated code into shared module.
- Consistent error messages and validation patterns.
- Run `cargo clippy --fix --allow-dirty` and `cargo fmt` before commits.