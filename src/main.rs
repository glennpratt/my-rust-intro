extern crate semver;

use semver::Version;


fn main() {
    hello_world();
    intro_semver();
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
