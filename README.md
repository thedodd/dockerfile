dockerfile
==========
A Rust library for dynamically generating Dockerfiles.

The use case this crate was originally built for was to build Docker images from a worker service running in Kubernetes for client workloads. This is definitely not the only pattern that is supported. The generated Dockerfiles could be persisted somewhere or discarded immediately after use. The containers generated are standard containers, built according to the Dockerfiles you generated.

### get started
First you will need to add this to your `Cargo.toml` dependencies.

```toml
dockerfile = "0.1"
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
This crate is in an early state, and there are a few additional things to be done with it. I would like to experiment with adding constructors for the various forms of instructions; EG, offer a constructor for `CMD` which takes an `impl Iterator<Item=AsRef<str>>` for building the form `CMD ["arg0", "arg1"]` &c. Not sure on that yet.
