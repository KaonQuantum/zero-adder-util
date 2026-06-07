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

## How it works

Only files whose names **end in digits** are renamed. The prefix (everything before the trailing number) is preserved.

For example, with `-n 2`:
- `track1` → `track001`
- `episode10` → `episode0010`
- `readme.txt` — skipped (doesn't end in a digit)

When removing zeros (`-d`), at least one digit is always kept — it will never reduce a number to an empty string.

## Install

```bash
cargo install --path .
```

## License

See [LICENSE](LICENSE).
