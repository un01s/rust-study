# rust study

## 00. get started

```
$ cargo new rust-study
$ cd rust-study
$ cargo run
Hello, world!
```

## 01. blinky

```
$ git checkout -b 01-blinky
```

### how to get the dependencies right?

the real trouble is to get the dependency right. the code is from [rp-hal-boards](https://github.com/rp-rs/rp-hal-boards). it uses workspace and another separate ```Cargo.toml``` for each board. Follow the code and refer to two ```Cargo.toml``` to add the dependency one after another until the code is compiled successfully. Try to use ```cargo add``` command or manually edit Cargo.toml. The last is to run the code for verification by ```cargo run```.

the implementation of critical_section is complained often by the compiler.


