# BitUnlocker - Project Index

## Root Files

| File | Description |
|------|-------------|
| `Cargo.toml` | Project manifest - defines binary name as `password-gen`, edition 2021, MIT/Apache-2.0 license |
| `README.md` | Comprehensive documentation covering installation, usage, template format, and all features |
| `AGENTS.md` | AI agent rules for Rust best practices (no dead code, proper error handling, testing standards) |

## Source Code (`src/`)

| File | Description |
|------|-------------|
| `lib.rs` | Library entry point - re-exports all modules for external use |
| `main.rs` | CLI binary - handles commands (`gen`, `unlock`, `help`) and argument parsing |
| `cache.rs` | Device-specific password caching using PowerShell to retrieve device serial number; saves to `.bitunlocker-cache-{device_id}.json` |
| `case.rs` | Case variation generation supporting 8 modes: lowercase, uppercase, mixed (title), all (2^N combinations), camelCase, PascalCase, snake_case, kebab-case, scream_SNAKE_CASE |
| `generator.rs` | Password generation engine - parses template parts and generates Cartesian product of all values; handles shortened/extended month placeholders |
| `leet.rs` | Leet-speak character substitutions for common characters (a→@,4, e→3, i→!1, l→1\|, o→0, s→$5, t→7) |
| `months.rs` | Month name list ordered from january to december for range generation |
| `numbers.rs` | Number range generation with zero-padding support (e.g., 001-999 format) |
| `template.rs` | Template parsing - extracts placeholders like `{number,min=X,max=Y}` and parses all properties |
| `unlock.rs` | BitLocker unlock functionality using PowerShell (`Unlock-BitLocker`) or `manage-bde.exe`; supports cache integration, stop-after-first option, and progress reporting |
| `words.rs` | Word manipulation functions: `generate_shortened()` creates subsequences by removing characters; `generate_extended()` creates longer words by duplicating/inserting characters |

## Test Files (`test/e2e/`)

| File | Description |
|------|-------------|
| `cache-test.spec.js` | Playwright integration tests for cache file creation, cached password skipping, and UUID cache persistence |
| `word-gen-test.spec.js` | Word generation functionality tests |
| `package.json` | NPM configuration with Playwright dependency for end-to-end testing |

## Module Dependencies

```
main.rs → lib.rs (via crate::)
    ↓
lib.rs exports:
  - cache (cache.rs) → get_device_serial_number(), DeviceCache, get_cache_file_path()
  - case (case.rs) → generate_case_variations()
  - generator (generator.rs) → parse_template(), TemplatePart, generate_passwords_from_parts()
  - leet (leet.rs) → apply_leet_variations(), get_leet_map()
  - months (months.rs) → get_month_order()
  - numbers (numbers.rs) → generate_number_range()
  - unlock (unlock.rs) → brute_force_unlock(), try_unlock_drive(), UnlockResult
  - words (words.rs) → generate_shortened(), generate_extended()
```

## Key Features Summary

1. **Template-based generation** - Customizable placeholders with properties like `min`, `max`, `begin`, `end`
2. **Case variations** - 8 mode options for case manipulation
3. **Leet-speak** - Character substitution for common letters
4. **Month ranges** - Generate passwords using month names in custom ranges
5. **Shortened words** - Remove characters to create subsequences
6. **Extended words** - Add duplicate/inserted characters to extend word length
7. **Device caching** - Skip previously-tested passwords on subsequent unlock attempts
8. **Unlock options** - PowerShell or manage-bde.exe, stop-after-first flag, cache control

## Usage Examples

```bash
# Generate passwords from template
password-gen gen "{month,min=1,max=5,begin=january,end=april,case=all}Example{number,min=001,max=999}"

# Attempt to unlock a drive
password-gen unlock D:

# With custom password file and cache disabled
password-gen unlock D: --passwords my_passwords.txt --no-cache