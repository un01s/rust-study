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

