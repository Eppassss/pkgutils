# Budowa pkgutils na Windows - Instrukcja

## Status
✅ **Projekt pomyślnie zbudowany i optymalizowany na Windows (bez WSL/Docker)**

## Wymagania
Wykonane:
- ✅ Rust 1.93.0
- ✅ Perl 5.32.1 (Strawberry Perl)
- ✅ NASM 2.15.05
- ✅ Visual C++ Build Tools (zainstalowane przez Rust)

## Wynik budowy

### Binary zoptymalizowany
- **Binary**: `target/release/pkg.exe`
- **Rozmiar**: 2.06 MB (2,161,664 bytes)
- **Redukcja**: 10.8% mniejszy od domyślnej budowy
- **Platforma**: Windows (x86_64-pc-windows-msvc)

### Optymalizacje zastosowane
- `opt-level = 3` - maksymalna optymalizacja kompilatora
- `lto = true` - Link Time Optimization
- `codegen-units = 1` - lepsze optymalizacje (wolniejsza kompilacja)
- `strip = true` - usunięcie symboli debuggingu

### Porównanie rozmiarów
```
Domyślna budowa:      2.31 MB (2,424,832 bytes)
Zoptymalizowana:      2.06 MB (2,161,664 bytes)
Oszczędzono:          263 KB (10.8%)
```

## Testy
### Unit testy: ✅ 10/10 passed
```
test package::tests::deserialize_no_depends ... ok
test package::tests::deserialize_empty_version ... ok
test package::tests::deserialize_with_depends ... ok
test package::tests::deserialize_repository ... ok
test package::tests::deserialize_repository_outdated ... ok
test package::tests::deserialize_empty_depends ... ok
test package::tests::deserialize_with_invalid_name_fails ... ok
test package::tests::deserialize_with_invalid_dependency_name_fails ... ok
test package::tests::package_name_split ... ok
test package::tests::roundtrip ... ok
```

### Integracyjne testy: ⏹️ Require Redox OS configuration
(Test wymaga środowiska Redox OS, na Windows zwraca `ValidRepoNotFound` - oczekiwane)

## Zmiany wprowadzone

### 1. Stubs dla bibliotek Unix-specyficznych
- `termion-stub/` - zastępuje bibliotekę termion
- `pkgar-stub/` - stub dla pkgar
- `pkgar-core-stub/` - stub dla pkgar-core
- `pkgar-keys-stub/` - stub dla pkgar-keys

### 2. Zamiana bibliotek
- `ignore` (Unix-only) → `walkdir` (cross-platform)
- `termion` → stub implementation
- `reqwest` features: `brotli` → `no_brotli()`

### 3. Modyfikacje kodu
**pkg-lib/src/recipes.rs**
- Zmieniono `ignore::Walk::new()` na `walkdir::WalkDir::new()`

**pkg-lib/src/library.rs**
- Dodano warunkową kompilację dla `pkgar_backend` (`#[cfg(target_os = "redox")]`)
- Na Windows zwraca `ValidRepoNotFound` (backend niedostępny)

**pkg-lib/src/net_backend/reqwest_backend.rs**
- Zmieniono `.brotli(true)` na `.connect_timeout()`
- Zmieniono `.brotli(false)` na `.no_brotli()`

**pkg-cli/src/main.rs**
- Usunięto `use termion`
- Zmieniono `IndicatifCallback` na `PlainCallback`

**pkg-lib/src/backend/mod.rs**
- Wyłączono `pkgar_backend` na Windows
- Usunięto `PkgarKeys` z Error enum

## Jak budować
```bash
# Ustawianie zmiennych środowiskowych
$env:PERL_EXECUTABLE="H:\DEV\Temp\perl\perl\bin\perl.exe"
$env:Path="H:\DEV\Temp\nasm-2.15.05;" + $env:Path

# Release build
cargo build --release

# Testy
cargo test --release

# Binary znajduje się w:
target/release/pkg.exe
```

## Ograniczenia na Windows
- Backend `pkgar` niedostępny (specyficzny dla Redox OS)
- Konfiguracja wymagana: `/tmp/pkg_install/etc/pkg.d/` dla repozytoriów
- Funkcjonalność ograniczona do operacji kompatybilnych z Windows

## Czyszczenie kodu
- ✅ Usunięto patch warningi (ring, cc-rs)
- ✅ Dodano `#[allow(dead_code)]` dla unused fields/methods (oczekiwane na Windows)
- ✅ 0 warningów w build

## CI/CD
- ✅ GitHub Actions workflow (`.github/workflows/windows-build.yml`)
  - Automatyczna budowa na Windows
  - Automatyczna budowa na Linux/macOS
  - Testy jednostkowe
  - Upload artefaktów

## Przyszłe ulepszenia
1. [ ] Skompletować support dla pkgar na Windows (jeśli potrzebny)
2. [ ] Dodać mock backend dla testów na Windows
3. [ ] Cross-platform dokumentacja
