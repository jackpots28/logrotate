# Logrotate - Unique Fork Written in Rust

_Taking inspiration from the Linux tool:
https://linux.die.net/man/8/logrotate_

---

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## Intent:

Slim version of a logrotate cli utility for: 
archiving, removing, and truncating files in a provided directory.

## Usage:

```
    logrotate --help
    Cli tool for rotating files within specified directory.
    
    Usage: logrotate [OPTIONS] --archive-method <ARCHIVE_METHOD> --directory <DIRECTORY> --keep-days <DAYS>
    
    Options:
          --dry-run
              Perform a dry run without making any changes Will output files marked for deletion, archival, and truncation
      -a, --archive-method <ARCHIVE_METHOD>
              Archival method to use [possible values: tar, tar-gunzip, zip]
      -d, --directory <DIRECTORY>
              Directory to parse through
      -k, --keep-days <DAYS>
              Number of days to keep archived files [default: 7]
      -h, --help
              Print help
      -V, --version
              Print version
```

---

## Support:

|    File Extension     | Supported | Planned |
|:---------------------:|:---------:|:-------:|
|      log / logs       |     ✅     |    -    |
|      txt / text       |     ✅     |    -    |
|          bin          |     ❌     |    ❌    |
|          xml          |     ✅     |    -    |
|          csv          |     ✅     |    -    |
|         json          |     ✅     |    -    |
|          elf          |     ❌     |    ❌    |
|          cef          |     ✅     |    -    |
|          clf          |     ✅     |    -    |
|        syslog         |     ✅      |    -    |
| Any Unnamed Extension |     ❌     |    ❓    |

---

## Build Locally:

_Requires Rust (minimum stable-2024 & any-architecture)_

```bash
    git clone https://github.com/jackpots28/logrotate.git
    cd logrotate
    cargo build --profile release
    sudo cp target/release/logrotate /usr/bin
```

Built binary is ~2Mb on disk

## Install Prebuilt:

Brew (MacOS):
```bash
    TBD
```

Linux (Fedora / CentOS / RHEL / Rocky - dnf or yum | *x86-64, aarch64, ppc64le*):
```bash
    TDB
```

Linux (Debian - apt | *x86-64, aarch64*):
```bash
    TBD
```

Windows (x86-64 assumed):
```bash
    TBD
```

Crates.io
```bash
    https://crates.io/crates/logrotate
```