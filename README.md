# BitUnlocker - BitLocker Password Recovery Tool

A command-line tool for generating password variations to recover access to BitLocker-encrypted drives (pendrives, USB sticks) when the recovery key is partially known.

## Features

- Full template-based password generation with customizable properties
- Month range generation (begin-end)
- Case variation control (lower/upper/mixed/all)
- Leet-speak character substitutions (@, 1, !, $, etc.)
- Number padding support (000-999 format)
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

### Template Format

The password generator uses templates with customizable properties:

**Available Properties:**

| Property | Description | Example |
|----------|-------------|---------|
| `maxSize=N` | Maximum character length | `maxSize=5` |
| `minSize=N` | Minimum character length | `minSize=3` |
| `begin=name` | Start of month range (optional) | `begin=january` |
| `end=name` | End of month range (optional) | `end=april` |
| `leetSpeak=true\|false` | Enable leet-speak substitutions | `leetSpeak=false` |
| `case=lower\|upper\|mixed\|all` | Case variation mode | `case=all` |

**Example Templates:**

```bash
# Full month range with case variations (default leetSpeak=false)
cargo run --release -- "{month,maxSize=5,minSize=3,begin=january,end=april,leetSpeak=false,case=all}Example{number,maxSize=3}"

# With leet-speak enabled
cargo run --release -- "{word,maxSize=4,minSize=2,leetSpeak=true,case=mixed}{number,maxSize=2}"
```

This generates passwords like:
- `APRILExample000`, `APRILExample123`
- `marchExample456`, `MARCHExample789`
- With leet-speak: `@prilExample001`, `M@rchExample123`

### Basic Usage

```bash
cargo run --release -- "{word,maxSize=5}Example{number,maxSize=3}"
```

Output is saved to `generated_passwords.txt`.

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `BITLOCKER_PENDRIVE` | Target drive path for BitLocker recovery | `D:\` |
| `OUTPUT_PASSWORD_FILE` | Output file path for generated passwords | `C:\passwords.txt` |

### Setting Environment Variables (Windows)

```cmd
set BITLOCKER_PENDRIVE=D:\
set OUTPUT_PASSWORD_FILE=C:\passwords.txt
```

### Using with D: Drive

If your pendrive is at D: and uses BitLocker:

1. First generate passwords:
   ```bash
   cargo run --release -- "{month,maxSize=5,begin=january,end=april,case=all}Example{number,maxSize=3}"
   ```

2. The password list will be in `generated_passwords.txt`

3. Try each password against the D: drive using Windows recovery options

## Template Examples

### Month Range with All Case Variations
```bash
cargo run --release -- "{month,maxSize=5,begin=january,end=december,case=all}Example{number,maxSize=3}"
```

### With Leet-Speak Enabled
```bash
cargo run --release -- "{word,maxSize=6,leetSpeak=true,case=mixed}{year,maxSize=4}"
```

### Simple Pattern with Numbers
```bash
cargo run --release -- "Password{number,maxSize=3}"
```
Generates: `Password000`, `Password001`, ..., `Password999`

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

## Performance Notes

- The output file can be very large (hundreds of thousands of passwords)
- Use appropriate `maxSize` and `minSize` values to limit the search space
- The month range (`begin-end`) helps reduce unnecessary combinations

## Author

**rodrigoazlima**

- GitHub: https://github.com/rodrigoazlima
- Website: https://rodrigoazlima.dev