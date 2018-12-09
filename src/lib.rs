#![cfg_attr(feature="docinclude", feature(external_doc))]
#![cfg_attr(feature="docinclude", doc(include="../README.md"))]

mod builder;
mod instructions;

pub use crate::{
    builder::{
        Dockerfile,
        DockerfileBuilder,
    },
    instructions::{
        Arg,
        Cmd,
        Copy,
        Directive,
        Entrypoint,
        From,
        Run,
        Instruction,
    },
};

//////////////////////////////////////////////////////////////////////////////////////////////////
// Unit Tests ////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dockerfile_generation_simple() {
        assert_eq!(Dockerfile::base("rust:1.30-slim")
            .finish().to_string(),
r##"FROM rust:1.30-slim
"##)
    }

    #[test]
    fn dockerfile_generation_with_directives_and_args() {
        assert_eq!(Dockerfile::base("rust:1.30-slim")
            .push_initial_directive(Directive::new("escape=`"))
            .push_initial_arg(Arg::new("TEST"))
            .push_initial_arg(Arg::new("OTHER=1"))
            .push(Copy::new("/static ./static".to_string()))
            .finish().to_string(),
r##"# escape=`
ARG TEST
ARG OTHER=1
FROM rust:1.30-slim
COPY /static ./static
"##)
    }

    #[test]
    fn dockerfile_readme_example() {
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

    #[test]
    fn dockerfile_with_run_and_cmd() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Run::new("echo 'Hello. Goodbye.' > /test-file"))
            .push(Cmd::new("cat /test-file"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
RUN echo 'Hello. Goodbye.' > /test-file
CMD cat /test-file
"##)
    }

    #[test]
    fn dockerfile_with_entrypoint_and_cmd() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Entrypoint::new("echo"))
            .push(Cmd::new("'Helooo! Goodbye!'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
ENTRYPOINT echo
CMD 'Helooo! Goodbye!'
"##)
    }
}
