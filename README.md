# BitUnlocker - BitLocker Password Recovery Tool

A command-line tool for generating password variations to recover access to BitLocker-encrypted drives (pendrives, USB sticks) when the recovery key is partially known.

## Features

- Full template-based password generation with customizable properties
- Month range generation (begin-end)
- Case variation control (lower/upper/mixed/all/camel/snake/kebab/scream)
- Leet-speak character substitutions (@, 1, !, $, etc.)
- Number padding support (000-999 format)
- Shortened placeholder - generate words with fewer characters by removing letters
- Extended placeholder - generate words with more characters by adding duplicates or new letters
- Generate large password lists efficiently

## Installation

### Prerequisites

- Rust toolchain (cargo and rustc)

If Rust is not installed:
```bash
winget install Rustlang.Rustup
```

### Build from Source

```bash
git clone https://github.com/rodrigoazlima/bitunlocker.git
cd bitunlocker
cargo build --release
```

## Usage

The CLI binary is named `password-gen`. Run it as `password-gen` (after building) or use `cargo run`.

### Template Format

Templates use placeholders with customizable properties:

**Available Properties:**

| Property | Description | Example |
|----------|-------------|---------|
| `min=X` | Minimum value for number/word range, or minimum length for shortened | `min=001` or `min=3` |
| `max=Y` | Maximum value for number/word range, or maximum length for extended | `max=333` or `max=10` |
| `begin=name` | Start of month range or source word for shortened/extended | `begin=january` or `begin=august` |
| `end=name` | End of month range (optional) | `end=december` |
| `leetSpeak=true\|false` | Enable leet-speak substitutions | `leetSpeak=false` |
| `case=lower\|upper\|mixed\|all\|camel\|snake\|kebab\|scream` | Case variation mode | `case=all` |
| `sep=_` or `-` | Separator for snake/kebab case modes | `sep=_` |

### Commands

| Command | Description |
|---------|-------------|
| `gen <template>` | Generate passwords from template and save to generated_passwords.txt |
| `unlock <drive>` | Try to unlock using existing generated_passwords.txt |
| `help` | Show help message |

**Example Templates:**

```bash
# Full month range with case variations (default leetSpeak=false)
password-gen gen "{month,min=1,max=3,begin=january,end=april,leetSpeak=false,case=all}Example{number,min=001,max=999}"

# With leet-speak enabled
password-gen gen "{word,min=1,max=6,leetSpeak=true,case=mixed}{number,min=00,max=99}"
```

This generates passwords like:
- `APRILExample000`, `APRILExample123`
- `marchExample456`, `MARCHExample789`
- With leet-speak: `@prilExample001`, `M@rchExample123`

### Basic Usage

```bash
password-gen gen "{word}Example{number,min=001,max=999}"
```

Output is saved to `generated_passwords.txt`.

## Shortened and Extended Placeholders

The shortened placeholder generates words by removing characters from the source word. The extended placeholder generates words by adding duplicate or new characters.

### Shortened Placeholder `{shortened}`

Generates all possible subsequences (combinations of characters in order) with a minimum length specified.

**Properties:**
- `min=X` - Minimum length of generated words (default: 1)
- `begin=word` - Source word to shorten from

**Examples:**

```bash
# Shortened versions of "august" with at least 3 characters
password-gen gen "{shortened,begin=august,min=3}"
```

This generates all combinations like:
- `aug`, `auu`, `aus`, `aut`, `agu`, `agt`, `gut`
- `augu`, `augs`, `augt`, `auus`, `augs`, ...
- `augst`, `auust`, `augus`, `augut`, `augst`, `august`

**For "august" (6 chars) with min=3:**
- `augst` - original without the second 'u'
- `auust` - original without the 'g'  
- `augus` - original without the 't'
- And many more combinations...

```bash
# With case variations and leet-speak
password-gen gen "{shortened,begin=august,min=3,case=all,leetSpeak=true}"
```

### Extended Placeholder `{extended}`

Generates words by adding duplicate characters or inserting new letters. The result is always longer than the source word.

**Properties:**
- `max=X` - Maximum length of generated words (default: source_length + 2)
- `begin=word` - Source word to extend from

**Examples:**

```bash
# Extended versions of "august" with max 10 characters
password-gen gen "{extended,begin=august,max=10}"
```

This generates words like:
- `aaugust`, `auugust`, `auggust`, `auguust`, `augusst`, `augustt`
- `aAugust`, `AuGust`, `AUguSt`, etc. (with case variations)
- With leet-speak: `@uu8ust`, `@ugU5t`, etc.

```bash
# Extended versions of "cat" with max 6 characters and all cases
password-gen gen "{extended,begin=cat,max=6,case=all}"
```

### Using Shortened/Extended with Months

Both placeholders work seamlessly with month names:

```bash
# Shortened versions of all months with at least 4 characters
password-gen gen "{shortened,begin=february,min=4}Example{number,min=001,max=99}"

# Extended versions of all months up to 12 characters
password-gen gen "{extended,begin=december,max=12}Example{number,min=001,max=99}"

# All case variations for shortened month names
password-gen gen "{shortened,begin=april,min=3,case=all}"
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `BITLOCKER_DRIVE` | Target drive path for BitLocker recovery | `D:\` |
| `OUTPUT_PASSWORD_FILE` | Output file path for generated passwords | `C:\passwords.txt` |

### Setting Environment Variables (Windows)

```cmd
set BITLOCKER_DRIVE=D:\
set OUTPUT_PASSWORD_FILE=C:\passwords.txt
```

### Using with D: Drive

If your pendrive is at D: and uses BitLocker:

1. First generate passwords:
   ```bash
   password-gen gen "{month,min=1,max=5,begin=january,end=april,case=all}Example{number,min=001,max=999}"
   ```

2. The password list will be in `generated_passwords.txt`

3. Try each password against the D: drive using Windows recovery options

## Case Variation Modes

| Mode | Description | Example (from "april") |
|------|-------------|------------------------|
| `lower` | All lowercase | `april` |
| `upper` | All uppercase | `APRIL` |
| `mixed` | First letter uppercase, rest lowercase | `April` |
| `all` | All 2^N combinations (uppercase/lowercase) | `aPrIl`, `ApRiL`, etc. |
| `camel` | First lowercase, rest uppercase | `aPRIL` |
| `snake` | Letters separated by underscores, all lowercase | `a_p_r_i_l` |
| `kebab` | Letters separated by hyphens, all lowercase | `a-p-r-i-l` |
| `scream` | Letters separated by underscores, all uppercase | `A_P_R_I_L` |

### Template Examples

#### Month Range with All Case Variations
```bash
password-gen gen "{month,min=1,max=5,begin=january,end=december,case=all}Example{number,min=001,max=999}"
```

#### Using camelCase mode
```bash
password-gen gen "{word,min=4,max=6,case=camel}Example{number,min=001,max=999}"
```
Generates: `aPRILExample001`, `mARChExample456`, etc.

#### Using snake_case mode
```bash
password-gen gen "{word,min=4,max=6,case=snake}Example{number,min=001,max=999}"
```
Generates: `a_p_r_i_lExample001`, `m_a_r_c_hExample456`, etc.

#### Using kebab-case mode
```bash
password-gen gen "{word,min=4,max=6,case=kebab}Example{number,min=001,max=999}"
```
Generates: `a-p-r-i-lExample001`, `m-a-r-c-hExample456`, etc.

#### Using SCREAM_SNAKE_CASE mode
```bash
password-gen gen "{word,min=4,max=6,case=scream}Example{number,min=001,max=999}"
```
Generates: `A_P_R_I_LExample001`, `M_A_R_C_HExample456`, etc.

### With Leet-Speak Enabled
```bash
password-gen gen "{word,min=1,max=6,leetSpeak=true,case=mixed}{number,min=1990,max=2030}"
```

### Simple Pattern with Numbers
```bash
password-gen gen "Password{number,min=001,max=999}"
```
Generates: `Password001`, `Password002`, ..., `Password999`

## Output

Passwords are written to `generated_passwords.txt` in the following format:
- One password per line
- Sorted by length (ascending), then alphabetically
- Numbers always padded with leading zeros (000-999)

Example output:
```
APRILExample000
aprilExample001
MARCHExample002
...
```

## Leet-Speak Substitutions

When enabled, the tool automatically generates common leet-speak variations:
- `a` → @, 4
- `i` → !, 1
- `l` → 1, |
- `o` → 0
- `s` → $, 5
- `t` → 7

## Testing

The project includes an end-to-end test suite for validating cache functionality.

### Test Location

Tests are located in the `test/e2e/` folder.

### Running Tests

```bash
cd test/e2e
npm install
npm test
```

### Test Coverage

The integration tests validate:

| Test | Description |
|------|-------------|
| **Cache File Creation** | Verifies no cache file created when using `--no-cache` flag |
| **Cached Password Skipping** | Validates cached passwords show "(cached)" and are skipped |
| **UUID Cache Persistence** | Tests that 10 cached passwords are skipped on second unlock with 20 total passwords |

### Cache Functionality

The tool uses a device-specific cache to track successfully unlocked passwords:

- **Cache File Location**: `.bitunlocker-cache-<device_id>.json` in the current working directory
- **Format**: One password per line, with `# Device ID: <serial>` header
- **Behavior**: Cached passwords are skipped during subsequent unlock operations

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `BITLOCKER_DRIVE` | Target drive path for BitLocker recovery | `D:\` |
| `OUTPUT_PASSWORD_FILE` | Output file path for generated passwords | `C:\passwords.txt` |

## Author

**rodrigoazlima**

- GitHub: https://github.com/rodrigoazlima
- Website: https://rodrigoazlima.dev