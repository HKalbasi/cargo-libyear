# `cargo libyear`

A simple measure of software dependency freshness. It is a single number telling you how up-to-date your dependencies are. See
[libyear](https://libyear.com)
for more info.

## Install

```
cargo install cargo-libyear
```

## Example

Basic usage:

```
$ cargo libyear
+------------------------------+-------------------------------+-------------------------+-----------------+
| Crate                        | Current Version               | Latest Version          | Libyears behind |
+------------------------------+-------------------------------+-------------------------+-----------------+
| addr2line                    | 0.21.0                        | 0.22.0                  | 0.66            |
| adler                        | 1.0.2                         | 1.0.2                   | 0.00            |
| always-assert                | 0.2.0                         | 0.2.0                   | 0.00            |
| anyhow                       | 1.0.80                        | 1.0.83                  | 0.21            |
| arbitrary                    | 1.3.2                         | 1.3.2                   | 0.00            |
| arrayvec                     | 0.7.4                         | 0.7.4                   | 0.00            |
| autocfg                      | 1.1.0                         | 1.3.0                   | 2.23            |
| backtrace                    | 0.3.69                        | 0.3.71                  | 0.58            |
| bitflags                     | 1.3.2                         | 2.5.0                   | 2.59            |
| bitflags                     | 2.4.2                         | 2.5.0                   | 0.17            |
# ... many more lines ...
Your system is 65.21 libyears behind
```

Sort by libyears to see most outdated dependencies first:

```
$ cargo libyear --sort libyear --top 10
+------------------------------+-------------------------------+-------------------------+-----------------+
| Crate                        | Current Version               | Latest Version          | Libyears behind |
+------------------------------+-------------------------------+-------------------------+-----------------+
| rand_chacha                  | 0.3.1                         | 0.9.0-alpha.1           | 2.77            |
| bitflags                     | 1.3.2                         | 2.5.0                   | 2.59            |
| autocfg                      | 1.1.0                         | 1.3.0                   | 2.23            |
| perf-event-open-sys          | 1.0.1                         | 4.0.0                   | 2.21            |
| wasi                         | 0.11.0+wasi-snapshot-preview1 | 0.13.0+wasi-0.2.0       | 2.14            |
| rand                         | 0.8.5                         | 0.9.0-alpha.1           | 2.09            |
| parking_lot                  | 0.12.1                        | 0.12.2                  | 1.91            |
| fixedbitset                  | 0.4.2                         | 0.5.7                   | 1.78            |
| inotify                      | 0.9.6                         | 0.10.2                  | 1.73            |
| perf-event                   | 0.4.7                         | 0.4.8                   | 1.53            |
+------------------------------+-------------------------------+-------------------------+-----------------+
Your system is 65.21 libyears behind
```

Show help with all available options:

```
$ cargo libyear --help
A simple measure of software dependency freshness

Usage: cargo-libyear [OPTIONS]

Options:
      --sort <SORT>
          Sort dependencies by the specified criteria

          Possible values:
          - alphabetical: Sort alphabetically by crate name (default)
          - libyear:      Sort by libyears behind (most outdated first)

          [default: alphabetical]

      --top <TOP>
          Show only the top N dependencies (when sorted by libyear, shows the most outdated)

      --manifest-path <MANIFEST_PATH>
          Path to Cargo.toml file

          [default: ./Cargo.toml]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Get the top 5 most outdated dependencies:

```
$ cargo libyear --sort libyear --top 5
```

Show all dependencies sorted alphabetically (default behavior):

```
$ cargo libyear --sort alphabetical
```
