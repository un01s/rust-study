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

The example code is [here](https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/src/bin/blinky.rs). Let's replace main.rs with this and start to add the dependency one by one.

```
$ cargo add panic-halt
$ cargo add rp2040-hal
$ cargo add embedded-hal
$ cargo add rp2040-boot2
```

Then run ```cargo build```, the complaint is as follows.

```
error[E0433]: failed to resolve: could not find `cortex_m_rt` in the list of imported crates
  --> src/main.rs:38:1
   |
38 | #[rp2040_hal::entry]
```

Let's add it.

```
cargo add cortex_m_rt
```

Another complaint:

```
error: linking with `rust-lld` failed: exit status: 1
...
= note: rust-lld: error: cannot find linker script defmt.x
```

Then it starts to complain about ```rust-lld: error: undefined symbol: _critical_section_1_0_release```. Check [this Cargo.toml in rp-hal](https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/Cargo.toml).

```
cargo add critical-section
cargo add defmt-rtt
```

Last, modify ```Cargo.toml``` with ```rp2040-hal``` to add features for critical-section-impl.

## 03 blinky with [lilos](https://github.com/cbiffle/lilos)

the example code is under ```examples/rp2040```.

Add obvious dependencies first.

```
$ cargo add lilos
$ cargo add panic-halt
$ cargo add rp2040-boot2
$ cargo add cortex-m-rt
$ cargo add cortex-m
$ cargo add rp2040-pac
```

## refs

* [cargo book: package manager](https://doc.rust-lang.org/cargo/)

* [cargo repo](https://github.com/rust-lang/cargo)

* [use cargo tree command](https://v5.chriskrycho.com/journal/using-cargo-tree/
)

