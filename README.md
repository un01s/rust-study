# rust study

## 00. get started

```
$ cargo new rust-study
$ cd rust-study
$ cargo run
Hello, world!
```

Just to get a random number.

```
$ cargo add rand
```

there are two complaints:

```
warning: use of deprecated function `rand::thread_rng`: Renamed to `rng`
6 |   let mut rng = rand::thread_rng();
  |                       ^^^^^^^^^^
warning: use of deprecated method `rand::Rng::gen`: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.
7 |   println!("Hello, rand {}", rng.gen::<f64>());
  |                                  ^^^
```

```
$ cargo run
Hello, rand 0.5185969356100997
```

## 01. blinky

```
$ git checkout -b 01-blinky
```

### how to get the dependencies right?

the real trouble is to get the dependency right. the code is from [rp-hal-boards](https://github.com/rp-rs/rp-hal-boards). it uses workspace and another separate Cargo.toml for each board. Follow the code and refer to two Cargo.toml to add the dependency one after another until the code is compiled successfully. Try to use cargo add command or manually edit Cargo.toml. The last is to run the code for verification by cargo run.

the implementation of critical_section is complained often by the compiler.

## 02. blinky again

this time is to try another example from [rp-hal](https://github.com/rp-rs/rp-hal).

```
$ git checkout -b 02-blinky
```

Let's show the dependency tree.

```
$ cargo tree
rust-study v0.1.0 (/Users/wb/proj/rust/rust-study)
├── cortex-m-rt v0.7.5
│   └── cortex-m-rt-macros v0.7.5 (proc-macro)
│       ├── proc-macro2 v1.0.101
│       │   └── unicode-ident v1.0.19
│       ├── quote v1.0.40
│       │   └── proc-macro2 v1.0.101 (*)
│       └── syn v2.0.106
│           ├── proc-macro2 v1.0.101 (*)
│           ├── quote v1.0.40 (*)
│           └── unicode-ident v1.0.19
├── critical-section v1.2.0
├── defmt v1.0.1
│   ├── bitflags v1.3.2
│   └── defmt-macros v1.0.1 (proc-macro)
│       ├── defmt-parser v1.0.0
│       │   └── thiserror v2.0.16
│       │       └── thiserror-impl v2.0.16 (proc-macro)
│       │           ├── proc-macro2 v1.0.101 (*)
│       │           ├── quote v1.0.40 (*)
│       │           └── syn v2.0.106 (*)
│       ├── proc-macro-error2 v2.0.1
│       │   ├── proc-macro-error-attr2 v2.0.0 (proc-macro)
│       │   │   ├── proc-macro2 v1.0.101 (*)
│       │   │   └── quote v1.0.40 (*)
│       │   ├── proc-macro2 v1.0.101 (*)
│       │   ├── quote v1.0.40 (*)
│       │   └── syn v2.0.106 (*)
│       ├── proc-macro2 v1.0.101 (*)
│       ├── quote v1.0.40 (*)
│       └── syn v2.0.106 (*)
├── defmt-rtt v1.0.0
│   ├── critical-section v1.2.0
│   └── defmt v1.0.1 (*)
├── embedded-hal v1.0.0
├── panic-halt v1.0.0
├── rp2040-boot2 v0.3.0
│   [build-dependencies]
│   └── crc-any v2.5.0
│       └── debug-helper v0.3.13
└── rp2040-hal v0.11.0
    ├── bitfield v0.14.0
    ├── cortex-m v0.7.7
    │   ├── bare-metal v0.2.5
    │   │   [build-dependencies]
    │   │   └── rustc_version v0.2.3
    │   │       └── semver v0.9.0
    │   │           └── semver-parser v0.7.0
    │   ├── bitfield v0.13.2
    │   ├── embedded-hal v0.2.7
    │   │   ├── nb v0.1.3
    │   │   │   └── nb v1.1.0
    │   │   └── void v1.0.2
    │   └── volatile-register v0.2.2
    │       └── vcell v0.1.3
    ├── critical-section v1.2.0
    ├── defmt v0.3.100
    │   └── defmt v1.0.1 (*)
    ├── embedded-dma v0.2.0
    │   └── stable_deref_trait v1.2.0
    ├── embedded-hal v0.2.7 (*)
    ├── embedded-hal v1.0.0
    ├── embedded-hal-async v1.0.0
    │   └── embedded-hal v1.0.0
    ├── embedded-hal-nb v1.0.0
    │   ├── embedded-hal v1.0.0
    │   └── nb v1.1.0
    ├── embedded-io v0.6.1
    ├── frunk v0.4.4
    │   ├── frunk_core v0.4.4
    │   └── frunk_derives v0.4.4 (proc-macro)
    │       ├── frunk_proc_macro_helpers v0.1.4
    │       │   ├── frunk_core v0.4.4
    │       │   ├── proc-macro2 v1.0.101 (*)
    │       │   ├── quote v1.0.40 (*)
    │       │   └── syn v2.0.106 (*)
    │       ├── quote v1.0.40 (*)
    │       └── syn v2.0.106 (*)
    ├── fugit v0.3.7
    │   └── gcd v2.3.0
    ├── itertools v0.10.5
    │   └── either v1.15.0
    ├── nb v1.1.0
    ├── paste v1.0.15 (proc-macro)
    ├── pio v0.2.1
    │   ├── arrayvec v0.7.6
    │   ├── num_enum v0.5.11
    │   │   └── num_enum_derive v0.5.11 (proc-macro)
    │   │       ├── proc-macro2 v1.0.101 (*)
    │   │       ├── quote v1.0.40 (*)
    │   │       └── syn v1.0.109
    │   │           ├── proc-macro2 v1.0.101 (*)
    │   │           ├── quote v1.0.40 (*)
    │   │           └── unicode-ident v1.0.19
    │   └── paste v1.0.15 (proc-macro)
    ├── rand_core v0.6.4
    ├── rp-binary-info v0.1.1
    ├── rp-hal-common v0.1.0
    │   └── fugit v0.3.7 (*)
    ├── rp2040-hal-macros v0.1.0 (proc-macro)
    │   ├── cortex-m-rt v0.7.5
    │   │   └── cortex-m-rt-macros v0.7.5 (proc-macro) (*)
    │   ├── proc-macro2 v1.0.101 (*)
    │   ├── quote v1.0.40 (*)
    │   └── syn v1.0.109 (*)
    ├── rp2040-pac v0.6.0
    │   ├── cortex-m v0.7.7 (*)
    │   ├── cortex-m-rt v0.7.5 (*)
    │   ├── critical-section v1.2.0
    │   └── vcell v0.1.3
    ├── usb-device v0.3.2
    │   ├── heapless v0.8.0
    │   │   ├── hash32 v0.3.1
    │   │   │   └── byteorder v1.5.0
    │   │   └── stable_deref_trait v1.2.0
    │   └── portable-atomic v1.11.1
    ├── vcell v0.1.3
    └── void v1.0.2
```

## refs

* [cargo book: package manager](https://doc.rust-lang.org/cargo/)

* [cargo repo](https://github.com/rust-lang/cargo)

* [use cargo tree command](https://v5.chriskrycho.com/journal/using-cargo-tree/
)

