# leading-zero-util

A CLI utility to add or remove leading zeros from the numbers at the end of filenames in a directory.

## Usage

```
leading-zero-util [OPTIONS] [PATH]
```

If no path is provided, it operates on the current directory.

## Options

| Flag | Short | Description |
|------|-------|-------------|
| `--number <N>` | `-n` | Number of zeros to add or remove (default: 1) |
| `--decrease` | `-d` | Remove leading zeros instead of adding them |
| `--basic-add` | `-b` | Add zeros naively without normalizing across files |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

## Examples

Add one leading zero to all files in the current directory:
```
leading-zero-util
```

Add two leading zeros to files in `~/my-folder`:
```
leading-zero-util ~/my-folder -n 2
```

Remove one leading zero from files in the current directory:
```
leading-zero-util -d
```

Remove two leading zeros:
```
leading-zero-util -d -n 2
```

Add one zero naively (without normalizing):
```
leading-zero-util -b
```

## How it works

Only files whose names **end in digits** are renamed. The prefix (everything before the trailing number) is preserved.

By default, adding zeros **normalizes** across all files — it finds the shortest numeric suffix and pads everything to the same width plus the requested count. For example, with `-n 1` on `problem-1` and `problem-11`:
- `problem-1` → `problem-01`
- `problem-11` → `problem-11` (already at target width)

With `--basic-add`, zeros are added naively without normalization:
- `problem-1` → `problem-01`
- `problem-11` → `problem-011`

When removing zeros (`-d`), at least one digit is always kept — it will never reduce a number to an empty string.

## Install

### Via crates.io (recommended)

If you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed:

```bash
cargo install leading-zero-util
```

### From source

Clone the repository and install locally:

```bash
git clone https://github.com/VidurThammineni/leading-zero-util
cd leading-zero-util
cargo install --path .
```

### Installing Cargo

If you don't have Cargo, install Rust (which includes Cargo) by following the instructions at https://www.rust-lang.org/tools/install.

Then re-run the install command above.

## License

See [LICENSE](LICENSE).
