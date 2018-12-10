dockerfile
==========
[![Build Status](https://travis-ci.org/thedodd/dockerfile.svg?branch=master)](https://travis-ci.org/thedodd/dockerfile)
[![Crates.io](https://img.shields.io/crates/v/dockerfile.svg)](https://crates.io/crates/dockerfile)
[![docs.rs](https://docs.rs/dockerfile/badge.svg)](https://docs.rs/dockerfile)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
![Crates.io](https://img.shields.io/crates/d/dockerfile.svg)
![Crates.io](https://img.shields.io/crates/dv/dockerfile.svg)
<!-- [![GitHub issues open](https://img.shields.io/github/issues-raw/thedodd/dockerfile.svg)]() -->
<!-- [![GitHub issues closed](https://img.shields.io/github/issues-closed-raw/thedodd/dockerfile.svg)]() -->

A Rust library for dynamically generating Dockerfiles.

The use case this crate was originally built for was to build Docker images from a worker service running in Kubernetes for client workloads. This is definitely not the only pattern that is supported. The generated Dockerfiles could be persisted somewhere or discarded immediately after use. The containers generated are standard containers, built according to the Dockerfiles you generated.

All of the Dockerfile instructions are supported in raw form as of 2018.12.09. There is an issue open to add more structured and type-safe interfaces for the instructions which need it.

### get started
First you will need to add this to your `Cargo.toml` dependencies.

```toml
dockerfile = "0.2"
```

Now you can start building Dockerfiles.

```rust
use dockerfile::{
    Dockerfile,
    Arg,
    Copy,
    Cmd,
};

fn main() {
    // Build up a new Dockerfile.
    let dockerfile = Dockerfile::base("rust:${RUST_VERSION}-slim")
        .push_initial_arg(Arg::new("RUST_VERSION=1.31"))
        .push(Copy::new("/static ./static"))
        .push(Cmd::new("echo 'Hello. Goodbye.'"))
        .finish();

    // Write it out as a string.
    let output = dockerfile.to_string();
    assert_eq!(output,
r##"ARG RUST_VERSION=1.31
FROM rust:${RUST_VERSION}-slim
COPY /static ./static
CMD echo 'Hello. Goodbye.'
"##)
}
```

### development
I would like to have this crate offer a type-safe interface for constructing the various Dockerfile instructions. This will help reduce bugs which could only be found once you actually attempt to invoke the docker build. I would like to experiment with adding constructors for the various forms of instructions; EG, offer a constructor for `CMD` which takes an `impl Iterator<Item=AsRef<str>>` for building the form `CMD ["arg0", "arg1"]` &c.
