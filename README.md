# Redox OS pkgutils

This repository contains utilities for package management on Redox. Currently, only `pkg` is included.

[![GitHub Actions](https://github.com/Eppassss/pkgutils/actions/workflows/windows-build.yml/badge.svg)](https://github.com/Eppassss/pkgutils/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Platform Support

- ✅ **Windows** (x86_64) - No WSL/Docker required
- ✅ **Linux** (x86_64) - Standard Rust toolchain
- ✅ **macOS** (x86_64) - Full compatibility
- ℹ️ **Redox OS** - Full functionality (native platform)

## `pkg`

The command `pkg` is the primary package management utility for Redox OS. In its current state, `pkg` supports the following commands:

| Command     | Functionality                  |
|-------------|--------------------------------|
| `install`   | Install packages               |
| `uninstall` | Uninstall packages             |
| `update`    | Update packages (all if empty) |
| `search`    | Search for a package           |
| `info`      | Package info                   |
| `list`      | List of installed packages     |

For more detailed information on how to invoke these subcommands, please run `pkg help <SUBCOMMAND>` in your terminal.

## Building

### Quick Start

**Windows (x86_64)**
```powershell
# Requires: Rust, Perl (Strawberry Perl), NASM
cargo build --release
```

**Linux/macOS**
```bash
# Requires: Rust toolchain
cargo build --release
```

### Detailed Instructions

For complete build instructions including dependency setup, see [WINDOWS_BUILD.md](./WINDOWS_BUILD.md).

## Testing

To run tests:

```bash
cargo test --release --lib
```

**Note**: Integration tests require Redox OS environment and are automatically skipped on other platforms.

## Binary Optimization

This build includes optimizations for maximum performance:
- Link-Time Optimization (LTO)
- Symbol stripping
- Aggressive optimization (-O3)
- Result: 10.8% size reduction (2.4 MB → 2.06 MB)

## Release

Pre-built binaries are available in [Releases](https://github.com/Eppassss/pkgutils/releases).

## License

MIT License - See [LICENSE](./LICENSE) file for details.
