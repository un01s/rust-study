extern crate rand;

use rand::Rng;

fn main() {
  let mut rng = rand::rng();
  println!("Hello, rand {}", rng.random::<f64>());
}
