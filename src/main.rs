extern crate semver;

use semver::Version;
use std::sync::{Arc,Mutex};


fn main() {
    hello_world();
    intro_semver();
    borrowed_ref();
    threads();
    locking();
    iterator();
}

fn hello_world() {
    println!("Hello, world!");
}

/// Couldn't call this semver because of the package name. Odd.
fn intro_semver() {
    assert!(Version::parse("1.2.3") == Ok(Version {
        major: 1u,
        minor: 2u,
        patch: 3u,
        pre: vec!(),
        build: vec!(),
    }));

        println!("Versions compared successfully!");
}

fn borrowed_ref() {
    let mut v = vec![];

    v.push("Hello");

    let x = &v[0].clone();

    v.push("world");

    println!("{}", x);
}

fn threads() {
    for i in range(0u, 10u) {
        spawn(proc() {
            println!("Hello, world! {}", i);
        });
    }
}

fn locking() {
    let numbers_mutex = Arc::new(Mutex::new(vec![1i, 2i, 3i]));

    for i in range(0u, 3u) {
        let nums_mutex = numbers_mutex.clone();
        spawn(proc() {
            let mut nums = nums_mutex.lock();
            for j in range(0, 3) { (*nums)[j] += 1 }
            println!("nums in thread {} is {}", i, *nums);
        });
    }
    let numbers = numbers_mutex.lock();
    println!("numbers in main {}", *numbers);
    // How do we join threads?
}

fn iterator() {
  // Rust inferred the type of 2 & 3 by 1i. Neat.
  let vec = vec![1i, 2, 3];

  for i in vec.iter() {
    println!("{}", i)
  }
}
