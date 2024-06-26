# `cargo libyear`

A simple measure of software dependency freshness. It is a single number telling you how up-to-date your dependencies are. See
[libyear](https://libyear.com)
for more info.

## Install

```
cargo install cargo-libyear
```

## Example

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
| byteorder                    | 1.5.0                         | 1.5.0                   | 0.00            |
| camino                       | 1.1.6                         | 1.1.6                   | 0.00            |
| cargo-platform               | 0.1.7                         | 0.1.8                   | 0.11            |
| cargo_metadata               | 0.18.1                        | 0.18.1                  | 0.00            |
| cc                           | 1.0.90                        | 1.0.97                  | 0.17            |
| cfg-if                       | 1.0.0                         | 1.0.0                   | 0.00            |
| chalk-derive                 | 0.96.0                        | 0.97.0                  | 0.25            |
| chalk-ir                     | 0.96.0                        | 0.97.0                  | 0.25            |
| chalk-recursive              | 0.96.0                        | 0.97.0                  | 0.25            |
| chalk-solve                  | 0.96.0                        | 0.97.0                  | 0.25            |
| command-group                | 2.1.0                         | 5.0.1                   | 0.71            |
| countme                      | 3.0.1                         | 3.0.1                   | 0.00            |
| cov-mark                     | 2.0.0-pre.1                   | 2.0.0-pre.1             | 0.00            |
| crc32fast                    | 1.4.0                         | 1.4.0                   | 0.00            |
| crossbeam-channel            | 0.5.12                        | 0.5.12                  | 0.00            |
| crossbeam-deque              | 0.8.5                         | 0.8.5                   | 0.00            |
| crossbeam-epoch              | 0.9.18                        | 0.9.18                  | 0.00            |
| crossbeam-utils              | 0.8.19                        | 0.8.19                  | 0.00            |
| ctrlc                        | 3.4.2                         | 3.4.4                   | 0.22            |
| dashmap                      | 5.5.3                         | 5.5.3                   | 0.00            |
| deranged                     | 0.3.11                        | 0.3.11                  | 0.00            |
| derive_arbitrary             | 1.3.2                         | 1.3.2                   | 0.00            |
| dissimilar                   | 1.0.7                         | 1.0.9                   | 0.81            |
| dot                          | 0.1.4                         | 0.1.4                   | 0.00            |
| drop_bomb                    | 0.1.5                         | 0.1.6                   | 1.15            |
| either                       | 1.10.0                        | 1.11.0                  | 0.17            |
| ena                          | 0.14.2                        | 0.14.3                  | 1.14            |
| equivalent                   | 1.0.1                         | 1.0.1                   | 0.00            |
| expect-test                  | 1.4.1                         | 1.5.0                   | 1.09            |
| filetime                     | 0.2.23                        | 0.2.23                  | 0.00            |
| fixedbitset                  | 0.4.2                         | 0.5.7                   | 1.78            |
| flate2                       | 1.0.28                        | 1.0.30                  | 0.55            |
| form_urlencoded              | 1.2.1                         | 1.2.1                   | 0.00            |
| fsevent-sys                  | 4.1.0                         | 4.1.0                   | 0.00            |
| fst                          | 0.4.7                         | 0.4.7                   | 0.00            |
| getrandom                    | 0.2.12                        | 0.2.15                  | 0.32            |
| gimli                        | 0.28.1                        | 0.29.0                  | 0.38            |
| hashbrown                    | 0.14.3                        | 0.14.5                  | 0.42            |
| heck                         | 0.4.1                         | 0.5.0                   | 1.11            |
| hermit-abi                   | 0.3.9                         | 0.3.9                   | 0.00            |
| home                         | 0.5.9                         | 0.5.9                   | 0.00            |
| idna                         | 0.5.0                         | 0.5.0                   | 0.00            |
| indexmap                     | 2.2.5                         | 2.2.6                   | 0.06            |
| inotify                      | 0.9.6                         | 0.10.2                  | 1.73            |
| inotify-sys                  | 0.1.5                         | 0.1.5                   | 0.00            |
| intern                       | 0.0.0                         | 0.2.0                   | 1.50            |
| itertools                    | 0.12.1                        | 0.12.1                  | 0.00            |
| itoa                         | 1.0.10                        | 1.0.11                  | 0.30            |
| jod-thread                   | 0.1.2                         | 0.1.2                   | 0.00            |
| kqueue                       | 1.0.8                         | 1.0.8                   | 0.00            |
| kqueue-sys                   | 1.0.4                         | 1.0.4                   | 0.00            |
| la-arena                     | 0.3.1                         | 0.3.1                   | 0.00            |
| la-arena                     | 0.3.1                         | 0.3.1                   | 0.00            |
| lazy_static                  | 1.4.0                         | 1.1.1                   | 0.38            |
| libc                         | 0.2.153                       | 0.2.154                 | 0.24            |
| libloading                   | 0.8.3                         | 0.8.3                   | 0.00            |
| libmimalloc-sys              | 0.1.35                        | 0.1.37                  | 0.61            |
| line-index                   | 0.1.1                         | 0.1.1                   | 0.00            |
| line-index                   | 0.1.1                         | 0.1.1                   | 0.00            |
| linked-hash-map              | 0.5.6                         | 0.5.6                   | 0.00            |
| lock_api                     | 0.4.11                        | 0.4.12                  | 0.52            |
| log                          | 0.4.21                        | 0.4.21                  | 0.00            |
| lsp-server                   | 0.7.6                         | 0.7.6                   | 0.00            |
| lsp-server                   | 0.7.6                         | 0.7.6                   | 0.00            |
| lsp-types                    | 0.95.0                        | 0.95.1                  | 0.27            |
| lz4_flex                     | 0.11.2                        | 0.11.3                  | 0.22            |
| memchr                       | 2.7.1                         | 2.7.2                   | 0.25            |
| memmap2                      | 0.5.10                        | 0.9.4                   | 0.92            |
| memoffset                    | 0.9.0                         | 0.9.1                   | 0.86            |
| mimalloc                     | 0.1.39                        | 0.1.41                  | 0.61            |
| miniz_oxide                  | 0.7.2                         | 0.7.2                   | 0.00            |
| mio                          | 0.8.11                        | 0.8.11                  | 0.00            |
| miow                         | 0.6.0                         | 0.6.0                   | 0.00            |
| nix                          | 0.26.4                        | 0.28.0                  | 0.49            |
| nix                          | 0.27.1                        | 0.28.0                  | 0.49            |
| nohash-hasher                | 0.2.0                         | 0.2.0                   | 0.00            |
| notify                       | 6.1.1                         | 6.1.1                   | 0.00            |
| nu-ansi-term                 | 0.49.0                        | 0.50.0                  | 0.50            |
| num-conv                     | 0.1.0                         | 0.1.0                   | 0.00            |
| num_cpus                     | 1.16.0                        | 1.16.0                  | 0.00            |
| object                       | 0.32.2                        | 0.35.0                  | 0.30            |
| object                       | 0.33.0                        | 0.35.0                  | 0.10            |
| once_cell                    | 1.19.0                        | 1.19.0                  | 0.00            |
| oorandom                     | 11.1.3                        | 11.1.3                  | 0.00            |
| parking_lot                  | 0.12.1                        | 0.12.2                  | 1.91            |
| parking_lot_core             | 0.9.9                         | 0.9.10                  | 0.52            |
| parser                       | 0.0.0                         | 0.0.0                   | 0.00            |
| paste                        | 1.0.14                        | 1.0.15                  | 0.81            |
| paths                        | 0.0.0                         | 0.0.0                   | 0.00            |
| percent-encoding             | 2.3.1                         | 2.3.1                   | 0.00            |
| perf-event                   | 0.4.7                         | 0.4.8                   | 1.53            |
| perf-event-open-sys          | 1.0.1                         | 4.0.0                   | 2.21            |
| petgraph                     | 0.6.4                         | 0.6.5                   | 0.71            |
| pin-project-lite             | 0.2.13                        | 0.2.14                  | 0.59            |
| powerfmt                     | 0.2.0                         | 0.2.0                   | 0.00            |
| ppv-lite86                   | 0.2.17                        | 0.2.17                  | 0.00            |
| proc-macro2                  | 1.0.78                        | 1.0.82                  | 0.29            |
| protobuf                     | 3.2.0                         | 3.4.0                   | 1.42            |
| protobuf-support             | 3.2.0                         | 3.4.0                   | 1.42            |
| pulldown-cmark               | 0.9.6                         | 0.10.3                  | 0.23            |
| pulldown-cmark-to-cmark      | 10.0.4                        | 13.0.0                  | 1.52            |
| quote                        | 1.0.35                        | 1.0.36                  | 0.27            |
| ra-ap-rustc_abi              | 0.44.0                        | 0.51.0                  | 0.13            |
| ra-ap-rustc_index            | 0.44.0                        | 0.51.0                  | 0.13            |
| ra-ap-rustc_index_macros     | 0.44.0                        | 0.51.0                  | 0.13            |
| ra-ap-rustc_lexer            | 0.44.0                        | 0.51.0                  | 0.13            |
| ra-ap-rustc_parse_format     | 0.44.0                        | 0.51.0                  | 0.13            |
| ra-ap-rustc_pattern_analysis | 0.44.0                        | 0.51.0                  | 0.13            |
| rand                         | 0.8.5                         | 0.9.0-alpha.1           | 2.09            |
| rand_chacha                  | 0.3.1                         | 0.9.0-alpha.1           | 2.77            |
| rand_core                    | 0.6.4                         | 0.9.0-alpha.1           | 1.51            |
| rayon                        | 1.9.0                         | 1.10.0                  | 0.07            |
| rayon-core                   | 1.12.1                        | 1.12.1                  | 0.00            |
| redox_syscall                | 0.4.1                         | 0.5.1                   | 0.52            |
| rowan                        | 0.15.15                       | 0.15.15                 | 0.00            |
| rustc-demangle               | 0.1.23                        | 0.1.24                  | 1.06            |
| rustc-hash                   | 1.1.0                         | 1.1.0                   | 0.00            |
| rustc_apfloat                | 0.2.0+llvm-462a31f5a5ab       | 0.2.0+llvm-462a31f5a5ab | 0.00            |
| ryu                          | 1.0.17                        | 1.0.18                  | 0.21            |
| same-file                    | 1.0.6                         | 1.0.6                   | 0.00            |
| scip                         | 0.3.3                         | 0.3.3                   | 0.00            |
| scoped-tls                   | 1.0.1                         | 1.0.1                   | 0.00            |
| scopeguard                   | 1.2.0                         | 1.2.0                   | 0.00            |
| semver                       | 1.0.22                        | 1.0.23                  | 0.21            |
| serde                        | 1.0.197                       | 1.0.201                 | 0.21            |
| serde_derive                 | 1.0.197                       | 1.0.201                 | 0.21            |
| serde_json                   | 1.0.114                       | 1.0.117                 | 0.21            |
| serde_repr                   | 0.1.18                        | 0.1.19                  | 0.27            |
| sharded-slab                 | 0.1.7                         | 0.1.7                   | 0.00            |
| smallvec                     | 1.13.1                        | 2.0.0-alpha.5           | 0.18            |
| smol_str                     | 0.2.1                         | 0.2.1                   | 0.00            |
| snap                         | 1.1.1                         | 1.1.1                   | 0.00            |
| stable_deref_trait           | 1.2.0                         | 1.2.0                   | 0.00            |
| syn                          | 2.0.52                        | 2.0.61                  | 0.19            |
| synstructure                 | 0.13.1                        | 0.13.1                  | 0.00            |
| text-size                    | 1.1.1                         | 1.1.1                   | 0.00            |
| thiserror                    | 1.0.57                        | 1.0.60                  | 0.23            |
| thiserror-impl               | 1.0.57                        | 1.0.60                  | 0.23            |
| thread_local                 | 1.1.8                         | 1.1.8                   | 0.00            |
| tikv-jemalloc-ctl            | 0.5.4                         | 0.5.4                   | 0.00            |
| tikv-jemalloc-sys            | 0.5.4+5.3.0-patched           | 0.5.4+5.3.0-patched     | 0.00            |
| tikv-jemallocator            | 0.5.4                         | 0.5.4                   | 0.00            |
| time                         | 0.3.34                        | 0.3.36                  | 0.18            |
| time-core                    | 0.1.2                         | 0.1.2                   | 0.00            |
| tinyvec                      | 1.6.0                         | 1.6.0                   | 0.00            |
| tinyvec_macros               | 0.1.1                         | 0.1.1                   | 0.00            |
| tracing                      | 0.1.40                        | 0.1.40                  | 0.00            |
| tracing-attributes           | 0.1.27                        | 0.1.27                  | 0.00            |
| tracing-core                 | 0.1.32                        | 0.1.32                  | 0.00            |
| tracing-log                  | 0.2.0                         | 0.2.0                   | 0.00            |
| tracing-subscriber           | 0.3.18                        | 0.3.18                  | 0.00            |
| tracing-tree                 | 0.3.0                         | 0.3.0                   | 0.00            |
| triomphe                     | 0.1.11                        | 0.1.11                  | 0.00            |
| typed-arena                  | 2.0.2                         | 2.0.2                   | 0.00            |
| ungrammar                    | 1.16.1                        | 1.16.1                  | 0.00            |
| unicase                      | 2.7.0                         | 2.7.0                   | 0.00            |
| unicode-bidi                 | 0.3.15                        | 0.3.15                  | 0.00            |
| unicode-ident                | 1.0.12                        | 1.0.12                  | 0.00            |
| unicode-normalization        | 0.1.23                        | 0.1.23                  | 0.00            |
| unicode-properties           | 0.1.1                         | 0.1.1                   | 0.00            |
| unicode-xid                  | 0.2.4                         | 0.2.4                   | 0.00            |
| url                          | 2.5.0                         | 2.5.0                   | 0.00            |
| valuable                     | 0.1.0                         | 0.1.0                   | 0.00            |
| version_check                | 0.9.4                         | 0.9.4                   | 0.00            |
| walkdir                      | 2.5.0                         | 2.5.0                   | 0.00            |
| wasi                         | 0.11.0+wasi-snapshot-preview1 | 0.13.0+wasi-0.2.0       | 2.14            |
| winapi                       | 0.3.9                         | 0.3.9                   | 0.00            |
| winapi-i686-pc-windows-gnu   | 0.4.0                         | 0.4.0                   | 0.00            |
| winapi-util                  | 0.1.6                         | 0.1.8                   | 0.60            |
| winapi-x86_64-pc-windows-gnu | 0.4.0                         | 0.4.0                   | 0.00            |
| windows-sys                  | 0.48.0                        | 0.52.0                  | 0.63            |
| windows-sys                  | 0.52.0                        | 0.52.0                  | 0.00            |
| windows-targets              | 0.48.5                        | 0.52.5                  | 0.65            |
| windows-targets              | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_aarch64_gnullvm      | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_aarch64_gnullvm      | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_aarch64_msvc         | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_aarch64_msvc         | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_i686_gnu             | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_i686_gnu             | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_i686_msvc            | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_i686_msvc            | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_x86_64_gnu           | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_x86_64_gnu           | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_x86_64_gnullvm       | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_x86_64_gnullvm       | 0.52.4                        | 0.52.5                  | 0.12            |
| windows_x86_64_msvc          | 0.48.5                        | 0.52.5                  | 0.65            |
| windows_x86_64_msvc          | 0.52.4                        | 0.52.5                  | 0.12            |
| write-json                   | 0.1.4                         | 0.1.4                   | 0.00            |
| xflags                       | 0.3.2                         | 0.4.0-pre.1             | 0.36            |
| xflags-macros                | 0.3.2                         | 0.4.0-pre.1             | 0.36            |
| xshell                       | 0.2.5                         | 0.2.6                   | 0.72            |
| xshell-macros                | 0.2.5                         | 0.2.6                   | 0.72            |
| xtask                        | 0.1.0                         | 0.1.0                   | 0.00            |
| zip                          | 0.6.6                         | 1.2.1                   | 0.98            |
+------------------------------+-------------------------------+-------------------------+-----------------+
Your system is 65.21 libyears behind
```
