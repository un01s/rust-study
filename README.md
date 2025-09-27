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

## refs

* [cargo book: package manager](https://doc.rust-lang.org/cargo/)

* [cargo repo](https://github.com/rust-lang/cargo)

* [use cargo tree command](https://v5.chriskrycho.com/journal/using-cargo-tree/
)

