# File hash checker

_Rapidly check file integrity using XXHASH-3 non-cryptographic hash algorithm_

---

## Compilation

### Both GUI and CLI versions

```bash
cargo build --release
```

### GUI version

```bash
cargo build --release -p file_hash_checker_gui
```

### CLI version

```bash
cargo build --release -p file_hash_checker_cli
```


## Usage

### GUI version

```bash
file_hash_checker_gui <file_to_hash>
```

### CLI version

```bash
file_hash_checker_cli <file_to_hash>
```