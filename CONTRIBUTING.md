contributing
============

### publishing releases
- Ensure the README has been updated to reflect accurate examples and installation version.
- Ensure the Cargo.toml version has been updated.
- Ensure docs are updated and rendering as needed.

### development
Running all tests for this system is pretty straightforward.

- cargo test --all-targets
- cargo +nightly test --doc --all-features

To visually check the built docs: `cargo +nightly doc --all-features --open`.
